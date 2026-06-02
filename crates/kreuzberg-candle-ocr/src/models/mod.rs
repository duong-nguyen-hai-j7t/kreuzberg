//! OCR model implementations.

#[cfg(feature = "trocr")]
pub mod trocr;

#[cfg(feature = "trocr")]
pub use trocr::{TrocrEngine, TrocrVariant};
