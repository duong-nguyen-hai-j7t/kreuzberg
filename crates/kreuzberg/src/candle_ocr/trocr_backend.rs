//! TrOCR backend plugin for the Kreuzberg OCR pipeline.
//!
//! This module wraps the candle-based TrOCR engine in the `OcrBackend` trait,
//! making it available to the extraction pipeline.

use async_trait::async_trait;
use std::borrow::Cow;
use std::path::Path;

use crate::Result;
use crate::core::config::OcrConfig;
use crate::plugins::{OcrBackend, OcrBackendType, Plugin};
use crate::types::ExtractionResult;
use kreuzberg_candle_ocr::DevicePreference;
use kreuzberg_candle_ocr::models::TrocrVariant;

/// TrOCR backend using candle transformers.
///
/// Recognizes text in images via Microsoft's TrOCR model. Supports printed
/// and handwritten text with four model variants (base/large × printed/handwritten).
///
/// # Configuration
///
/// TrOCR accepts backend options for runtime tuning:
/// ```json
/// {
///   "variant": "base-printed",
///   "device": "auto"
/// }
/// ```
///
/// - `variant` (string): `"base-printed"` (default), `"large-printed"`, `"base-handwritten"`, `"large-handwritten"`
/// - `device` (string): `"auto"`, `"cpu"`, `"cuda"`, `"metal"`
#[cfg_attr(alef, alef(skip))]
pub struct TrocrBackend {
    variant: TrocrVariant,
}

impl TrocrBackend {
    /// Create a new TrOCR backend with the specified variant.
    pub fn new(variant: TrocrVariant) -> Self {
        Self { variant }
    }

    /// Create a TrOCR backend with the default variant (base-printed).
    pub fn default_variant() -> Self {
        Self::new(TrocrVariant::default())
    }

    /// Parse backend options to extract TrOCR-specific configuration.
    fn parse_options(config: &OcrConfig) -> (TrocrVariant, DevicePreference) {
        let mut variant = TrocrVariant::default();
        let mut device = DevicePreference::default();

        if let Some(opts) = &config.backend_options {
            // Parse variant preference
            if let Some(v) = opts.get("variant").and_then(|v| v.as_str()) {
                variant = match v {
                    "large-printed" => TrocrVariant::LargePrinted,
                    "base-handwritten" => TrocrVariant::BaseHandwritten,
                    "large-handwritten" => TrocrVariant::LargeHandwritten,
                    _ => TrocrVariant::BasePrinted, // default on unknown
                };
            }

            // Parse device preference
            if let Some(d) = opts.get("device").and_then(|v| v.as_str()) {
                device = match d {
                    "cpu" => DevicePreference::Cpu,
                    "cuda" => DevicePreference::Cuda,
                    "metal" => DevicePreference::Metal,
                    _ => DevicePreference::Auto,
                };
            }
        }

        (variant, device)
    }
}

impl Plugin for TrocrBackend {
    fn name(&self) -> &str {
        "candle-trocr"
    }

    fn version(&self) -> String {
        "0.1.0".to_string()
    }

    fn initialize(&self) -> Result<()> {
        tracing::debug!("Initializing TrOCR backend: {}", self.variant.description());
        Ok(())
    }

    fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl OcrBackend for TrocrBackend {
    async fn process_image(&self, image_bytes: &[u8], config: &OcrConfig) -> Result<ExtractionResult> {
        // Parse configuration
        let (_variant, device) = Self::parse_options(config);

        // Validate image data
        if image_bytes.is_empty() {
            return Err(crate::KreuzbergError::Validation {
                message: "Empty image data provided to TrOCR".to_string(),
                source: None,
            });
        }

        // In Phase 3b, this will:
        // 1. Select device (CPU/CUDA/Metal)
        // 2. Load the engine from cache or HF Hub
        // 3. Process the image through the encoder-decoder pipeline
        // 4. Decode the output tokens to text
        // 5. Return confidence from token logits
        //
        // For now, stub out the interface to validate the harness works.
        let content = format!(
            "TrOCR {} processing not yet implemented (device: {:?})",
            self.variant, device
        );

        Ok(ExtractionResult {
            content,
            mime_type: Cow::Borrowed("text/plain"),
            ..Default::default()
        })
    }

    async fn process_image_file(&self, path: &Path, config: &OcrConfig) -> Result<ExtractionResult> {
        let bytes = crate::core::io::read_file_async(path).await?;
        self.process_image(&bytes, config).await
    }

    fn supports_language(&self, lang: &str) -> bool {
        // TrOCR base-printed is trained primarily on English and works best
        // on English text. Other variants may support other languages but
        // comprehensive support requires additional fine-tuning.
        matches!(lang, "eng" | "en")
    }

    fn supported_languages(&self) -> Vec<String> {
        vec!["eng".to_string(), "en".to_string()]
    }

    fn backend_type(&self) -> OcrBackendType {
        OcrBackendType::Candle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trocr_backend_creation() {
        let backend = TrocrBackend::default_variant();
        assert_eq!(backend.name(), "candle-trocr");
        assert_eq!(backend.backend_type(), OcrBackendType::Candle);
    }

    #[test]
    fn test_trocr_language_support() {
        let backend = TrocrBackend::default_variant();
        assert!(backend.supports_language("eng"));
        assert!(backend.supports_language("en"));
        assert!(!backend.supports_language("deu"));
        assert!(!backend.supports_language("fra"));
    }

    #[test]
    fn test_trocr_supported_languages() {
        let backend = TrocrBackend::default_variant();
        let langs = backend.supported_languages();
        assert!(langs.contains(&"eng".to_string()));
        assert!(langs.contains(&"en".to_string()));
    }

    #[test]
    fn test_parse_options_defaults() {
        let config = OcrConfig::default();
        let (variant, device) = TrocrBackend::parse_options(&config);
        assert_eq!(variant, TrocrVariant::BasePrinted);
        assert_eq!(device, DevicePreference::Auto);
    }

    #[test]
    fn test_parse_options_custom_variant() {
        let mut config = OcrConfig::default();
        config.backend_options = Some(serde_json::json!({
            "variant": "large-printed"
        }));
        let (variant, _device) = TrocrBackend::parse_options(&config);
        assert_eq!(variant, TrocrVariant::LargePrinted);
    }

    #[test]
    fn test_parse_options_custom_device() {
        let mut config = OcrConfig::default();
        config.backend_options = Some(serde_json::json!({
            "device": "cpu"
        }));
        let (_variant, device) = TrocrBackend::parse_options(&config);
        assert_eq!(device, DevicePreference::Cpu);
    }

    #[test]
    fn test_initialize_and_shutdown() {
        let backend = TrocrBackend::default_variant();
        assert!(backend.initialize().is_ok());
        assert!(backend.shutdown().is_ok());
    }
}
