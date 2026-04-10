//! Page rendering using the pdf_oxide backend.
//!
//! Provides page-to-image rendering via pdf_oxide's `rendering` module (pure Rust,
//! tiny-skia based). This is the oxide equivalent of the pdfium-based `rendering.rs`.
//!
//! Feature-gated on `pdf-oxide-rendering` which enables both `pdf-oxide` and
//! `pdf_oxide/rendering`.

use super::OxideDocument;
use crate::pdf::error::{PdfError, Result};
use image::DynamicImage;

/// Points per inch in PDF coordinate space.
const PDF_POINTS_PER_INCH: f64 = 72.0;

/// Default DPI for page rendering. 150 balances legibility for vision-model
/// OCR against memory and file size.
const DEFAULT_RENDER_DPI: u32 = 150;

/// Maximum image dimension (width or height) in pixels. Prevents OOM on
/// extremely large pages at high DPI.
const MAX_IMAGE_DIMENSION: u32 = 65536;

/// Minimum DPI clamp.
const MIN_DPI: u32 = 72;

/// Maximum DPI clamp.
const MAX_DPI: u32 = 600;

/// Render a single PDF page to a `DynamicImage` using pdf_oxide.
///
/// Automatically adjusts DPI to stay within `MAX_IMAGE_DIMENSION` for
/// oversized pages, matching the pdfium renderer's auto-adjust logic.
///
/// # Arguments
///
/// * `doc` - Mutable reference to the oxide document
/// * `page_index` - Zero-based page index
/// * `dpi` - Target DPI (defaults to 150 if `None`)
///
/// # Returns
///
/// An RGB `DynamicImage` suitable for OCR or display.
pub(crate) fn render_page_to_image(
    doc: &mut OxideDocument,
    page_index: usize,
    dpi: Option<u32>,
) -> Result<DynamicImage> {
    let target_dpi = dpi.unwrap_or(DEFAULT_RENDER_DPI);

    // Get page dimensions (in points) for DPI auto-adjust
    let (page_width_pts, page_height_pts) = match doc.doc.get_page_media_box(page_index) {
        Ok((llx, lly, urx, ury)) => ((urx - llx).abs() as f64, (ury - lly).abs() as f64),
        Err(_) => (612.0, 792.0), // Letter size fallback
    };

    let adjusted_dpi = calculate_optimal_dpi(
        page_width_pts,
        page_height_pts,
        target_dpi,
        MAX_IMAGE_DIMENSION,
        MIN_DPI,
        MAX_DPI,
    );

    let options = pdf_oxide::rendering::RenderOptions::with_dpi(adjusted_dpi);

    let rendered = pdf_oxide::rendering::render_page(&mut doc.doc, page_index, &options)
        .map_err(|e| PdfError::RenderingFailed(format!("pdf_oxide render page {page_index}: {e}")))?;

    // Convert raw PNG/JPEG bytes to DynamicImage via the image crate
    let cursor = std::io::Cursor::new(&rendered.data);
    let img = image::io::Reader::new(cursor)
        .with_guessed_format()
        .map_err(|e| PdfError::RenderingFailed(format!("Failed to guess image format: {e}")))?
        .decode()
        .map_err(|e| PdfError::RenderingFailed(format!("Failed to decode rendered image: {e}")))?;

    Ok(img)
}

/// Render all pages of a PDF to `DynamicImage` instances.
///
/// # Arguments
///
/// * `doc` - Mutable reference to the oxide document
/// * `dpi` - Target DPI (defaults to 150 if `None`)
///
/// # Returns
///
/// A vector of RGB `DynamicImage` instances, one per page.
pub(crate) fn render_all_pages(doc: &mut OxideDocument, dpi: Option<u32>) -> Result<Vec<DynamicImage>> {
    let page_count = doc
        .doc
        .page_count()
        .map_err(|e| PdfError::RenderingFailed(format!("pdf_oxide: failed to get page count: {e}")))?;

    let mut images = Vec::with_capacity(page_count);
    for page_idx in 0..page_count {
        let img = render_page_to_image(doc, page_idx, dpi)?;
        images.push(img);
    }

    Ok(images)
}

/// Render a single page to PNG-encoded bytes.
///
/// Convenience wrapper matching the pdfium `render_pdf_page_to_png` signature.
pub(crate) fn render_page_to_png(doc: &mut OxideDocument, page_index: usize, dpi: Option<u32>) -> Result<Vec<u8>> {
    let img = render_page_to_image(doc, page_index, dpi)?;
    encode_png(&img)
}

/// Encode a `DynamicImage` as PNG bytes.
fn encode_png(image: &DynamicImage) -> Result<Vec<u8>> {
    use image::GenericImageView;
    let (w, h) = image.dimensions();
    let estimated = (w as usize * h as usize * 3) / 2;
    let mut buf = std::io::Cursor::new(Vec::with_capacity(estimated));
    image
        .write_to(&mut buf, image::ImageFormat::Png)
        .map_err(|e| PdfError::RenderingFailed(format!("PNG encoding failed: {e}")))?;
    Ok(buf.into_inner())
}

/// Calculate optimal DPI that keeps both dimensions within `max_dimension`.
///
/// Mirrors the pdfium renderer's auto-adjust logic.
fn calculate_optimal_dpi(
    page_width_pts: f64,
    page_height_pts: f64,
    target_dpi: u32,
    max_dimension: u32,
    min_dpi: u32,
    max_dpi: u32,
) -> u32 {
    let width_inches = page_width_pts / PDF_POINTS_PER_INCH;
    let height_inches = page_height_pts / PDF_POINTS_PER_INCH;

    let width_at_target = (width_inches * target_dpi as f64) as u32;
    let height_at_target = (height_inches * target_dpi as f64) as u32;

    if width_at_target <= max_dimension && height_at_target <= max_dimension {
        return target_dpi.clamp(min_dpi, max_dpi);
    }

    let width_limited_dpi = (max_dimension as f64 / width_inches) as u32;
    let height_limited_dpi = (max_dimension as f64 / height_inches) as u32;

    width_limited_dpi.min(height_limited_dpi).clamp(min_dpi, max_dpi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_optimal_dpi_within_limits() {
        let dpi = calculate_optimal_dpi(612.0, 792.0, 300, 65536, 72, 600);
        assert!((72..=600).contains(&dpi));
    }

    #[test]
    fn test_calculate_optimal_dpi_oversized_page() {
        let dpi = calculate_optimal_dpi(10000.0, 10000.0, 300, 4096, 72, 600);
        assert!(dpi >= 72);
        assert!(dpi < 300);
    }

    #[test]
    fn test_calculate_optimal_dpi_min_clamp() {
        let dpi = calculate_optimal_dpi(100.0, 100.0, 10, 65536, 72, 600);
        assert_eq!(dpi, 72);
    }

    #[test]
    fn test_calculate_optimal_dpi_max_clamp() {
        let dpi = calculate_optimal_dpi(100.0, 100.0, 1000, 65536, 72, 600);
        assert_eq!(dpi, 600);
    }

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_RENDER_DPI, 150);
        assert_eq!(MAX_IMAGE_DIMENSION, 65536);
        assert_eq!(MIN_DPI, 72);
        assert_eq!(MAX_DPI, 600);
    }
}
