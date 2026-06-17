//! DeepSeek-OCR inference engine for generation loop integration.
//!
//! Plugs the DeepSeekOCRModel into the `generate_mrope` generation loop for token decoding.

use candle_core::Device;
use candle_nn::VarBuilder;

use crate::error::Result;

use super::{config::DeepseekOCRConfig, model::DeepseekOCRModel, processor::DeepseekOCRProcessor};

/// DeepSeek-OCR inference engine.
///
/// Manages model initialization, input processing, and generation.
///
/// TODO: Wire into `generate_mrope` from `models/glm_ocr/mtp.rs`.
#[derive(Debug)]
pub struct DeepseekOCREngine {
    /// Inference model.
    model: DeepseekOCRModel,
    /// Input processor.
    processor: DeepseekOCRProcessor,
    /// Computation device.
    device: Device,
    /// Model version.
    version: usize,
}

impl DeepseekOCREngine {
    /// Create a new DeepSeek-OCR engine.
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::CandleOcrError`] if model loading or initialization
    /// fails.
    pub fn new(
        vb: VarBuilder,
        config: DeepseekOCRConfig,
        device: &Device,
        version: usize,
    ) -> Result<Self> {
        let processor = DeepseekOCRProcessor::new(device, vb.dtype(), version)?;
        let model = DeepseekOCRModel::new(vb, config, version)?;
        Ok(Self {
            model,
            processor,
            device: device.clone(),
            version,
        })
    }

    /// Return the inference model (for integration with generation loop).
    #[must_use]
    pub fn model(&mut self) -> &mut DeepseekOCRModel {
        &mut self.model
    }

    /// Return the input processor.
    #[must_use]
    pub fn processor(&self) -> &DeepseekOCRProcessor {
        &self.processor
    }

    /// Return the computation device.
    #[must_use]
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Return the model version.
    #[must_use]
    pub fn version(&self) -> usize {
        self.version
    }
}
