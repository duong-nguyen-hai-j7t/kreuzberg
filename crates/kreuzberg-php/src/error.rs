//! Error conversion from Rust to PHP exceptions
//!
//! Converts `KreuzbergError` from the Rust core into appropriate PHP exceptions.

use ext_php_rs::prelude::*;

/// ValidationException - Raised when validation fails
#[php_class]
#[extends(ext_php_rs::exception::PhpException)]
pub struct ValidationException;

#[php_impl]
impl ValidationException {}

/// ParsingException - Raised when document parsing fails
#[php_class]
#[extends(ext_php_rs::exception::PhpException)]
pub struct ParsingException;

#[php_impl]
impl ParsingException {}

/// OcrException - Raised when OCR processing fails
#[php_class]
#[extends(ext_php_rs::exception::PhpException)]
pub struct OcrException;

#[php_impl]
impl OcrException {}

/// MissingDependencyException - Raised when required dependency is missing
#[php_class]
#[extends(ext_php_rs::exception::PhpException)]
pub struct MissingDependencyException;

#[php_impl]
impl MissingDependencyException {}

/// CacheException - Raised when cache operations fail
#[php_class]
#[extends(ext_php_rs::exception::PhpException)]
pub struct CacheException;

#[php_impl]
impl CacheException {}

/// ImageProcessingException - Raised when image processing fails
#[php_class]
#[extends(ext_php_rs::exception::PhpException)]
pub struct ImageProcessingException;

#[php_impl]
impl ImageProcessingException {}

/// PluginException - Raised when plugin operations fail
#[php_class]
#[extends(ext_php_rs::exception::PhpException)]
pub struct PluginException;

#[php_impl]
impl PluginException {}

/// Convert Rust KreuzbergError to PHP exception.
///
/// Maps error variants to appropriate PHP exception types:
/// - `Validation` → `ValidationException`
/// - `UnsupportedFormat` → `ValidationException`
/// - `Parsing` → `ParsingException`
/// - `Io` → PHP's standard Exception
/// - `Ocr` → `OcrException`
/// - `Plugin` → `PluginException`
/// - `LockPoisoned` → PHP's RuntimeException
/// - `Cache` → `CacheException`
/// - `ImageProcessing` → `ImageProcessingException`
/// - `Serialization` → `ParsingException`
/// - `MissingDependency` → `MissingDependencyException`
/// - `Other` → PHP's RuntimeException
pub fn to_php_exception(error: kreuzberg::KreuzbergError) -> PhpException {
    use kreuzberg::KreuzbergError;

    let message = format_error_message(&error);

    match error {
        KreuzbergError::Validation { .. } => PhpException::from_class::<ValidationException>(message),
        KreuzbergError::UnsupportedFormat(_) => PhpException::from_class::<ValidationException>(message),
        KreuzbergError::Parsing { .. } => PhpException::from_class::<ParsingException>(message),
        KreuzbergError::Io(_) => PhpException::default(message),
        KreuzbergError::Ocr { .. } => PhpException::from_class::<OcrException>(message),
        KreuzbergError::Plugin { .. } => PhpException::from_class::<PluginException>(message),
        KreuzbergError::LockPoisoned(_) => PhpException::default(format!("Lock poisoned: {}", message)),
        KreuzbergError::Cache { .. } => PhpException::from_class::<CacheException>(message),
        KreuzbergError::ImageProcessing { .. } => PhpException::from_class::<ImageProcessingException>(message),
        KreuzbergError::Serialization { .. } => PhpException::from_class::<ParsingException>(message),
        KreuzbergError::MissingDependency(_) => PhpException::from_class::<MissingDependencyException>(message),
        KreuzbergError::Other(_) => PhpException::default(message),
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
