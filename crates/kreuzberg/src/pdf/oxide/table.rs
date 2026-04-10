//! Table detection using the pdf_oxide backend.
//!
//! Extracts word-level text with bounding boxes from PDF pages via pdf_oxide
//! spans, then feeds them into the backend-agnostic table reconstruction
//! pipeline (`table_reconstruct`).

use super::OxideDocument;
use crate::pdf::error::Result;
use crate::pdf::hierarchy::SegmentData;
use crate::pdf::table_reconstruct::{HocrWord, split_segment_to_words};

/// Minimum word length for table detection (filter out noise).
const MIN_WORD_LENGTH: usize = 1;

/// Extract words with positions from a PDF page for table detection.
///
/// Uses pdf_oxide's span extraction to get text with font metadata, then
/// splits spans into individual words with proportional bounding boxes.
/// The resulting `HocrWord` vector can be fed directly into `reconstruct_table`.
///
/// # Arguments
///
/// * `doc` - Mutable reference to the oxide document
/// * `page_index` - Zero-based page index
/// * `min_confidence` - Minimum confidence threshold (0.0-100.0). PDF text uses 95.0.
///
/// # Returns
///
/// Vector of `HocrWord` objects with text and bounding box information.
pub(crate) fn extract_words_from_page(
    doc: &mut OxideDocument,
    page_index: usize,
    min_confidence: f64,
) -> Result<Vec<HocrWord>> {
    // Get page height for coordinate conversion (PDF y=0 at bottom → image y=0 at top)
    let page_height = doc
        .doc
        .get_page_media_box(page_index)
        .ok()
        .map(|(_, lly, _, ury)| (ury - lly).abs())
        .unwrap_or(792.0); // Letter size fallback

    let spans = match doc.doc.extract_spans(page_index) {
        Ok(spans) => spans,
        Err(e) => {
            tracing::debug!(page = page_index, "pdf_oxide extract_spans failed for table: {e}");
            return Ok(Vec::new());
        }
    };

    let confidence = 95.0;
    if confidence < min_confidence {
        return Ok(Vec::new());
    }

    let mut words: Vec<HocrWord> = Vec::new();

    for span in spans {
        // Skip artifacts (headers/footers/watermarks)
        if span.artifact_type.is_some() {
            continue;
        }

        let text = span.text.trim();
        if text.is_empty() {
            continue;
        }

        let is_bold = span.font_weight == pdf_oxide::layout::text_block::FontWeight::Bold;
        let bbox = &span.bbox;

        // Convert from screen coords (y=0 at top) to PDF coords (y=0 at bottom)
        let screen_bottom = bbox.y + bbox.height;
        let pdf_baseline_y = page_height - screen_bottom;
        let pdf_y = page_height - bbox.y - bbox.height;

        let seg = SegmentData {
            text: span.text.clone(),
            x: bbox.x,
            y: pdf_y,
            width: bbox.width,
            height: bbox.height,
            font_size: span.font_size,
            is_bold,
            is_italic: span.is_italic,
            is_monospace: span.font_name.contains("Mono")
                || span.font_name.contains("Courier")
                || span.font_name.contains("Consola"),
            baseline_y: pdf_baseline_y,
        };

        // Split multi-word segments into individual HocrWords
        let segment_words = split_segment_to_words(&seg, page_height);
        for word in segment_words {
            if word.text.len() >= MIN_WORD_LENGTH {
                words.push(word);
            }
        }
    }

    Ok(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_word_length_constant() {
        assert_eq!(MIN_WORD_LENGTH, 1);
    }
}
