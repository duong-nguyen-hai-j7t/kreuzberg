//! Render document pages to PNG rasters for vision calls.
//!
//! TODO(Wave 1, agent A): implement `pages_for_call(bytes, mime, mode) -> Result<Vec<PageImage>>`.
//! Reuse [`crate::pdf::render::render_pdf_page_to_png`] for PDFs and the `image` crate for raster
//! images; wrap blocking work in `tokio::task::spawn_blocking`; honour [`super::VisionConfig::dpi`].
//! Drop the cloud `PersistContext`/storage coupling entirely. Port the cloud `#[cfg(test)]` suite.
