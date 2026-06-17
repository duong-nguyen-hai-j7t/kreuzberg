//! Fuse OCR bounding boxes onto extracted fields to produce citations.
//!
//! TODO(Wave 1, agent D): implement `fuse(merged, ocr_elements, emit_citations) -> CitationEnvelope`
//! producing [`super::CitedField`]/[`super::CitationSource`] provenance. Reuse the cloud verbatim
//! fuzzy text match (>0.8 char ratio). Read kreuzberg's typed [`crate::types::OcrElement`] via an
//! adapter that serializes each element to `{ text, page_number, bbox: [x, y, width, height] }`
//! (map `OcrBoundingGeometry::Rectangle { left, top, width, height }` → `[left, top, width, height]`
//! as `f64`) so the fuzzy match logic ports unchanged. Port the cloud tests.
