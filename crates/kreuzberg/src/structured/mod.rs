//! LLM-driven structured extraction orchestrator.
//!
//! This module turns a document plus a [`PresetSpec`] (an extraction schema + prompt) into
//! validated structured JSON. It runs the regular kreuzberg extraction pipeline, decides whether
//! vision (page rasters) is needed via [`crate::heuristics::choose_call_mode`], rasterizes pages
//! lazily, packs them into token-aware batches, calls a vision-capable LLM, schema-validates and
//! merges the responses, optionally fuses OCR bounding boxes as citations, and assembles a
//! [`StructuredOutput`].
//!
//! The mechanism lives here; domain knowledge (preset catalogs, tuned thresholds, prompt bodies,
//! model selection, a distributed cache) is supplied by the caller through [`StructuredOptions`],
//! [`PresetSpec::Inline`], [`crate::presets::Registry::extend_from_dir`], and the
//! [`VisionCallCache`] trait.
//!
//! Requires the `structured` feature and is unavailable on `wasm32` (needs native HTTP and PDF
//! rendering).
//!
//! # Layout
//!
//! - [`rasterize`] — render PDF/image pages to PNG ([`PageImage`]).
//! - [`chunker`] — token-aware batch packing.
//! - [`vision_client`] — vision LLM request/response adapter over `liter-llm`.
//! - [`postprocess`] — JSON Schema validation + multi-batch merge.
//! - [`citations`] — fuse OCR bounding boxes onto extracted fields.
//! - [`prompt`] — build system/user prompts from a resolved preset.
//! - [`cache`] — the [`VisionCallCache`] trait and an in-process Moka implementation.

pub mod cache;
pub mod chunker;
pub mod citations;
pub mod postprocess;
pub mod prompt;
pub mod rasterize;
pub mod vision_client;

use std::collections::BTreeMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::core::config::LlmConfig;
use crate::heuristics::{ExtractionConfidence, StructuredCallMode, StructuredThresholds};
use crate::types::LlmUsage;

pub use cache::{CacheKey, VisionCallCache};

/// A single rendered document page, ready to send to a vision model.
#[derive(Debug, Clone)]
pub struct PageImage {
    /// 1-indexed page number within the source document.
    pub page_number: u32,
    /// PNG-encoded page raster.
    pub png_bytes: Vec<u8>,
}

/// How the extraction schema + prompt are supplied.
#[derive(Debug, Clone)]
pub enum PresetSpec {
    /// Look the preset up by id in the global [`crate::presets::Registry`].
    Named(String),
    /// Use a caller-provided preset directly (boxed; presets carry an embedded JSON schema).
    Inline(Box<crate::presets::Preset>),
}

/// Vision-call tuning. Defaults are conservative starting points; production callers override them.
#[derive(Debug, Clone)]
pub struct VisionConfig {
    /// Rasterization DPI for PDF pages.
    pub dpi: u32,
    /// Maximum output tokens requested per vision call.
    pub max_output_tokens: u32,
    /// Confidence below which `TextOnlyWithVisionFallback` escalates to a vision call.
    pub fallback_threshold: f32,
    /// Token ceiling for a single batched call (input side).
    pub max_input_tokens: u32,
    /// Maximum characters of extracted text included as a prompt excerpt.
    pub max_excerpt_chars: usize,
    /// Optional model override; when `None` the model comes from [`StructuredOptions::llm`].
    pub model: Option<String>,
    /// Sampling temperature.
    pub temperature: f32,
}

impl Default for VisionConfig {
    fn default() -> Self {
        Self {
            dpi: 200,
            max_output_tokens: 8000,
            fallback_threshold: 0.6,
            max_input_tokens: 800_000,
            max_excerpt_chars: 200_000,
            model: None,
            temperature: 0.0,
        }
    }
}

/// Runtime options for a structured-extraction call.
///
/// This is a runtime call type, not a serializable config: it can carry a non-serializable
/// [`VisionCallCache`] trait object. Config-file users construct it from the serializable
/// [`crate::core::config::StructuredExtractionConfig`].
#[derive(Debug, Clone)]
pub struct StructuredOptions {
    /// LLM connection config (model, key, base URL). Reused via [`crate::llm::client`].
    pub llm: LlmConfig,
    /// Call-mode decision thresholds.
    pub thresholds: StructuredThresholds,
    /// Force a specific call mode, bypassing the heuristic.
    pub force_call_mode: Option<StructuredCallMode>,
    /// Context variables substituted into the preset's `context_template`.
    pub context: BTreeMap<String, String>,
    /// Optional vision-call cache. Excluded from the FFI/binding surface.
    pub cache: Option<Arc<dyn VisionCallCache>>,
    /// Maximum concurrent vision calls.
    pub max_parallel_calls: usize,
    /// Vision-call tuning.
    pub vision: VisionConfig,
    /// Override the preset's citation setting; `None` defers to `preset.emit_citations`.
    pub emit_citations: Option<bool>,
}

impl Default for StructuredOptions {
    fn default() -> Self {
        Self {
            llm: LlmConfig::default(),
            thresholds: StructuredThresholds::default(),
            force_call_mode: None,
            context: BTreeMap::new(),
            cache: None,
            max_parallel_calls: 4,
            vision: VisionConfig::default(),
            emit_citations: None,
        }
    }
}

/// Where a cited field's value came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationSource {
    /// Value taken from the LLM response only.
    Llm,
    /// Value taken from extracted (OCR) text only.
    Extracted,
    /// Value present in both and reconciled.
    Fused,
    /// No citation could be attached.
    None,
}

/// A single extracted field with optional provenance (page + bounding box + confidence).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitedField {
    /// The field value.
    pub value: serde_json::Value,
    /// 1-indexed source page, when known.
    pub page: Option<u32>,
    /// Bounding box `[x, y, width, height]` in page pixels, when known.
    pub bbox: Option<[f64; 4]>,
    /// Citation confidence in `0.0..=1.0`, when known.
    pub confidence: Option<f64>,
    /// Provenance of the value.
    pub source: CitationSource,
}

/// The structured result in both cited (nested) and flattened (value-only) shapes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationEnvelope {
    /// Citation-annotated structured output (fields wrapped as [`CitedField`] when citations emit).
    pub structured_output: serde_json::Value,
    /// Flat value-only structured output.
    pub flat: serde_json::Value,
}

/// The result of a structured-extraction run.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StructuredOutput {
    /// The extracted document text (excerpt used for the prompt / text-only mode).
    pub content: String,
    /// Citation-annotated structured output.
    pub structured_output: CitationEnvelope,
    /// Flattened value-only structured output (mirrors `structured_output.flat`).
    pub structured_output_flat: serde_json::Value,
    /// Confidence scoring for this run.
    pub confidence: ExtractionConfidence,
    /// Per-call LLM token usage.
    pub llm_usage: Vec<LlmUsage>,
    /// The call mode actually used.
    pub call_mode_used: StructuredCallMode,
    /// Whether a vision fallback fired after a low-confidence text-only pass.
    pub fallback_used: bool,
    /// Resolved preset id.
    pub preset_id: String,
    /// Resolved preset version.
    pub preset_version: String,
}

/// Errors returned by the structured-extraction entry points.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StructuredError {
    /// Named preset not found in the registry.
    #[error("preset not found: {0}")]
    PresetNotFound(String),
    /// Preset resolution (schema/prompt templating) failed.
    #[error("preset resolution failed: {0}")]
    Resolve(String),
    /// Underlying document extraction failed.
    #[error("document extraction failed: {0}")]
    Extraction(String),
    /// Page rasterization failed.
    #[error("page rasterization failed: {0}")]
    Rasterize(String),
    /// A vision LLM call failed.
    #[error("vision call failed: {0}")]
    Vision(String),
    /// Schema validation failed.
    #[error("schema validation failed: {0}")]
    Schema(String),
    /// Every vision batch failed; no partial result could be assembled.
    #[error("all vision batches failed: {0}")]
    AllBatchesFailed(String),
    /// The document mime type cannot be structurally extracted.
    #[error("structured extraction is not supported for mime type: {0}")]
    UnsupportedMime(String),
}
