//! Error conversion from Rust to PHP exceptions
//!
//! Converts `KreuzbergError` from the Rust core into appropriate PHP exceptions.

use ext_php_rs::prelude::*;

/// Convert Rust KreuzbergError to PHP exception.
///
/// Maps all error variants to PHP's standard Exception with descriptive messages
/// that include the error type prefix for categorization.
pub fn to_php_exception(error: kreuzberg::KreuzbergError) -> PhpException {
    use kreuzberg::KreuzbergError;

    let message = format_error_message(&error);

    match error {
        KreuzbergError::Validation { .. } => PhpException::default(format!("[Validation] {}", message)),
        KreuzbergError::UnsupportedFormat(_) => PhpException::default(format!("[UnsupportedFormat] {}", message)),
        KreuzbergError::Parsing { .. } => PhpException::default(format!("[Parsing] {}", message)),
        KreuzbergError::Io(_) => PhpException::default(format!("[IO] {}", message)),
        KreuzbergError::Ocr { .. } => PhpException::default(format!("[OCR] {}", message)),
        KreuzbergError::Plugin { .. } => PhpException::default(format!("[Plugin] {}", message)),
        KreuzbergError::LockPoisoned(_) => PhpException::default(format!("[LockPoisoned] {}", message)),
        KreuzbergError::Cache { .. } => PhpException::default(format!("[Cache] {}", message)),
        KreuzbergError::ImageProcessing { .. } => PhpException::default(format!("[ImageProcessing] {}", message)),
        KreuzbergError::Serialization { .. } => PhpException::default(format!("[Serialization] {}", message)),
        KreuzbergError::MissingDependency(_) => PhpException::default(format!("[MissingDependency] {}", message)),
        KreuzbergError::Other(_) => PhpException::default(format!("[Other] {}", message)),
    }
}

/// Format error message with source chain.
fn format_error_message(error: &kreuzberg::KreuzbergError) -> String {
    use kreuzberg::KreuzbergError;

    match error {
        KreuzbergError::Validation { message, source } => {
            if let Some(src) = source {
                format!("{}: {}", message, src)
            } else {
                message.clone()
            }
        }
        KreuzbergError::UnsupportedFormat(msg) => msg.clone(),
        KreuzbergError::Parsing { message, source } => {
            if let Some(src) = source {
                format!("{}: {}", message, src)
            } else {
                message.clone()
            }
        }
        KreuzbergError::Io(e) => e.to_string(),
        KreuzbergError::Ocr { message, source } => {
            if let Some(src) = source {
                format!("{}: {}", message, src)
            } else {
                message.clone()
            }
        }
        KreuzbergError::Plugin { message, plugin_name } => {
            format!("Plugin error in '{}': {}", plugin_name, message)
        }
        KreuzbergError::LockPoisoned(msg) => msg.clone(),
        KreuzbergError::Cache { message, source } => {
            if let Some(src) = source {
                format!("{}: {}", message, src)
            } else {
                message.clone()
            }
        }
        KreuzbergError::ImageProcessing { message, source } => {
            if let Some(src) = source {
                format!("{}: {}", message, src)
            } else {
                message.clone()
            }
        }
        KreuzbergError::Serialization { message, source } => {
            if let Some(src) = source {
                format!("{}: {}", message, src)
            } else {
                message.clone()
            }
        }
        KreuzbergError::MissingDependency(msg) => msg.clone(),
        KreuzbergError::Other(msg) => msg.clone(),
    }
}
