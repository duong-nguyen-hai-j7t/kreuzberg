//! Integration tests for PaddleOCR functionality.
//!
//! These tests require:
//! - Network access to download models from HuggingFace
//! - ONNX Runtime installed on the system
//!
//! Run with: `cargo test -p kreuzberg --features paddle-ocr --test paddle_ocr_integration -- --ignored`

#![cfg(feature = "paddle-ocr")]

use std::path::PathBuf;

use kreuzberg::core::config::OcrConfig;
use kreuzberg::paddle_ocr::{ModelManager, PaddleOcrBackend, PaddleOcrConfig};
use kreuzberg::plugins::OcrBackend;
use kreuzberg::types::ExtractionResult;

/// Helper to get the test documents directory
fn test_documents_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("test_documents")
}

/// Helper to get a temporary cache directory for tests
fn test_cache_dir() -> PathBuf {
    std::env::temp_dir().join("kreuzberg_paddle_test")
}

/// Test that model manager can download models from HuggingFace.
///
/// This test downloads actual models and verifies they are cached correctly.
/// It's ignored by default since it requires network access and ~100MB download.
#[tokio::test]
#[ignore = "requires network access and ~100MB download"]
async fn test_model_download_from_huggingface() {
    let cache_dir = test_cache_dir();

    // Clean up any existing cache
    let _ = std::fs::remove_dir_all(&cache_dir);

    let manager = ModelManager::new(cache_dir.clone());

    // Verify cache is empty
    assert!(!manager.are_models_cached());

    // Download models (synchronous now)
    let result = manager.ensure_models_exist();
    assert!(result.is_ok(), "Model download failed: {:?}", result.err());

    let paths: kreuzberg::paddle_ocr::ModelPaths = result.unwrap();

    // Verify all ONNX files exist
    assert!(paths.det_model.exists(), "Detection model not found");
    assert!(paths.cls_model.exists(), "Classification model not found");
    assert!(paths.rec_model.exists(), "Recognition model not found");

    // Verify files have ONNX extension
    assert_eq!(paths.det_model.extension().unwrap(), "onnx");
    assert_eq!(paths.cls_model.extension().unwrap(), "onnx");
    assert_eq!(paths.rec_model.extension().unwrap(), "onnx");

    // Verify cache reports correctly
    assert!(manager.are_models_cached());

    // Check cache stats
    let stats = manager.cache_stats().unwrap();
    assert_eq!(stats.model_count, 3);
    // Models should be > 1MB each
    assert!(stats.total_size_bytes > 1_000_000);

    println!("Cache stats: {:?}", stats);
    println!("Detection model: {:?}", paths.det_model);
    println!("Classification model: {:?}", paths.cls_model);
    println!("Recognition model: {:?}", paths.rec_model);
}

/// Test OCR on a simple English "Hello World" image.
///
/// This test requires ONNX Runtime and downloaded models.
#[tokio::test]
#[ignore = "requires ONNX Runtime and downloaded models"]
async fn test_ocr_hello_world_english() {
    let image_path = test_documents_dir().join("images/test_hello_world.png");
    assert!(image_path.exists(), "Test image not found: {:?}", image_path);

    let image_bytes = std::fs::read(&image_path).expect("Failed to read image");

    let config = PaddleOcrConfig::new("en").with_cache_dir(test_cache_dir());

    let backend = PaddleOcrBackend::with_config(config).expect("Failed to create backend");

    let ocr_config = OcrConfig {
        backend: "paddle-ocr".to_string(),
        language: "en".to_string(),
        ..Default::default()
    };

    let result: kreuzberg::Result<ExtractionResult> = backend.process_image(&image_bytes, &ocr_config).await;
    assert!(result.is_ok(), "OCR failed: {:?}", result.err());

    let extraction: ExtractionResult = result.unwrap();
    let text = extraction.content.to_lowercase();

    println!("OCR result: {}", extraction.content);

    // Should contain "hello" and "world"
    assert!(
        text.contains("hello") || text.contains("helo"),
        "Expected 'hello' in OCR result: {}",
        text
    );
    assert!(
        text.contains("world") || text.contains("worid"),
        "Expected 'world' in OCR result: {}",
        text
    );
}

/// Test OCR on a complex English document (newspaper).
#[tokio::test]
#[ignore = "requires ONNX Runtime and downloaded models"]
async fn test_ocr_newspaper_english() {
    let image_path = test_documents_dir().join("images/ocr_image.jpg");
    assert!(image_path.exists(), "Test image not found: {:?}", image_path);

    let image_bytes = std::fs::read(&image_path).expect("Failed to read image");

    let config = PaddleOcrConfig::new("en").with_cache_dir(test_cache_dir());

    let backend = PaddleOcrBackend::with_config(config).expect("Failed to create backend");

    let ocr_config = OcrConfig {
        backend: "paddle-ocr".to_string(),
        language: "en".to_string(),
        ..Default::default()
    };

    let result: kreuzberg::Result<ExtractionResult> = backend.process_image(&image_bytes, &ocr_config).await;
    assert!(result.is_ok(), "OCR failed: {:?}", result.err());

    let extraction: ExtractionResult = result.unwrap();
    let text = extraction.content.to_uppercase();

    println!(
        "OCR result (first 500 chars): {}",
        &extraction.content[..extraction.content.len().min(500)]
    );

    // Should contain "NASDAQ" and "AMEX" from the header
    assert!(
        text.contains("NASDAQ") || text.contains("NASOAQ"),
        "Expected 'NASDAQ' in OCR result"
    );
    assert!(
        text.contains("AMEX") || text.contains("STOCK"),
        "Expected 'AMEX' or 'STOCK' in OCR result"
    );
}

/// Test OCR on Chinese text.
#[tokio::test]
#[ignore = "requires ONNX Runtime and downloaded models"]
async fn test_ocr_chinese_text() {
    let image_path = test_documents_dir().join("images/chi_sim_image.jpeg");
    assert!(image_path.exists(), "Test image not found: {:?}", image_path);

    let image_bytes = std::fs::read(&image_path).expect("Failed to read image");

    // Use Chinese language setting
    let config = PaddleOcrConfig::new("ch").with_cache_dir(test_cache_dir());

    let backend = PaddleOcrBackend::with_config(config).expect("Failed to create backend");

    let ocr_config = OcrConfig {
        backend: "paddle-ocr".to_string(),
        language: "ch".to_string(),
        ..Default::default()
    };

    let result: kreuzberg::Result<ExtractionResult> = backend.process_image(&image_bytes, &ocr_config).await;
    assert!(result.is_ok(), "OCR failed: {:?}", result.err());

    let extraction: ExtractionResult = result.unwrap();

    println!("OCR result: {}", extraction.content);

    // Should contain some Chinese characters
    let has_chinese = extraction.content.chars().any(|c| {
        let c = c as u32;
        (0x4E00..=0x9FFF).contains(&c) // CJK Unified Ideographs
    });

    assert!(has_chinese, "Expected Chinese characters in OCR result");
}

/// Test that the backend correctly reports supported languages.
#[test]
fn test_supported_languages() {
    let backend = PaddleOcrBackend::new().expect("Failed to create backend");

    // Direct PaddleOCR codes
    assert!(backend.supports_language("ch"));
    assert!(backend.supports_language("en"));
    assert!(backend.supports_language("japan"));
    assert!(backend.supports_language("korean"));

    // Mapped Tesseract/ISO codes
    assert!(backend.supports_language("chi_sim"));
    assert!(backend.supports_language("eng"));
    assert!(backend.supports_language("jpn"));
    assert!(backend.supports_language("fra"));
    assert!(backend.supports_language("deu"));

    // Unsupported
    assert!(!backend.supports_language("xyz"));
    assert!(!backend.supports_language("klingon"));
}

/// Test that empty image returns an error.
#[tokio::test]
async fn test_empty_image_error() {
    let backend = PaddleOcrBackend::new().expect("Failed to create backend");

    let ocr_config = OcrConfig {
        backend: "paddle-ocr".to_string(),
        language: "en".to_string(),
        ..Default::default()
    };

    let result: kreuzberg::Result<ExtractionResult> = backend.process_image(&[], &ocr_config).await;
    assert!(result.is_err(), "Expected error for empty image");
}

/// Test that invalid image data returns an error (requires ONNX Runtime).
#[tokio::test]
#[ignore = "requires ONNX Runtime and downloaded models"]
async fn test_invalid_image_error() {
    let config = PaddleOcrConfig::new("en").with_cache_dir(test_cache_dir());
    let backend = PaddleOcrBackend::with_config(config).expect("Failed to create backend");

    let ocr_config = OcrConfig {
        backend: "paddle-ocr".to_string(),
        language: "en".to_string(),
        ..Default::default()
    };

    // Random bytes that aren't a valid image
    let invalid_bytes = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let result: kreuzberg::Result<ExtractionResult> = backend.process_image(&invalid_bytes, &ocr_config).await;
    assert!(result.is_err(), "Expected error for invalid image data");
}

/// Test processing an image file directly.
#[tokio::test]
#[ignore = "requires ONNX Runtime and downloaded models"]
async fn test_process_file() {
    let image_path = test_documents_dir().join("images/test_hello_world.png");
    assert!(image_path.exists(), "Test image not found: {:?}", image_path);

    let config = PaddleOcrConfig::new("en").with_cache_dir(test_cache_dir());
    let backend = PaddleOcrBackend::with_config(config).expect("Failed to create backend");

    let ocr_config = OcrConfig {
        backend: "paddle-ocr".to_string(),
        language: "en".to_string(),
        ..Default::default()
    };

    let result: kreuzberg::Result<ExtractionResult> = backend.process_file(&image_path, &ocr_config).await;
    assert!(result.is_ok(), "OCR failed: {:?}", result.err());

    let extraction: ExtractionResult = result.unwrap();
    let text = extraction.content.to_lowercase();

    assert!(
        text.contains("hello") || text.contains("helo"),
        "Expected 'hello' in OCR result"
    );
}

/// Test that explicit cache_dir in config overrides default.
#[test]
fn test_cache_dir_explicit_config() {
    // Set explicit config - this should always work regardless of env vars
    let config = PaddleOcrConfig::new("en").with_cache_dir(PathBuf::from("/explicit/path"));
    let resolved = config.resolve_cache_dir();

    // Explicit config should always win
    assert_eq!(resolved, PathBuf::from("/explicit/path"));
}

/// Test default cache directory when no explicit config is set.
#[test]
fn test_cache_dir_default() {
    // Save and clear env var to test default behavior
    let original = std::env::var("KREUZBERG_PADDLE_CACHE_DIR").ok();

    // SAFETY: This is a test that manipulates environment variables.
    // Tests should be run with --test-threads=1 if this causes issues.
    unsafe {
        std::env::remove_var("KREUZBERG_PADDLE_CACHE_DIR");
    }

    let config = PaddleOcrConfig::new("en");
    let resolved = config.resolve_cache_dir();

    // Default should use .kreuzberg/paddle-ocr relative to CWD
    assert!(resolved.to_string_lossy().contains(".kreuzberg"));
    assert!(resolved.to_string_lossy().contains("paddle-ocr"));

    // Restore
    unsafe {
        if let Some(val) = original {
            std::env::set_var("KREUZBERG_PADDLE_CACHE_DIR", val);
        }
    }
}
