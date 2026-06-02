//! Candle-based VLM OCR backends.
//!
//! Pure-Rust transformer OCR via the `kreuzberg-candle-ocr` crate. This module
//! holds the `OcrBackend + Plugin` impls and the per-model configuration
//! plumbing; model code itself lives in `kreuzberg-candle-ocr::models`.
//!
//! ## Status
//!
//! Phase 3a: TrOCR backend implemented behind `candle-trocr` feature.
//! Phase 3b-d: GOT-OCR 2.0, GLM-OCR, PaddleOCR-VL are added in subsequent
//! phases behind their respective sub-features on `kreuzberg-candle-ocr`.

mod config;

#[cfg(feature = "candle-trocr")]
pub mod trocr_backend;

pub use config::{CandleModelId, CandleOcrConfig};

#[cfg(feature = "candle-trocr")]
pub use trocr_backend::TrocrBackend;
