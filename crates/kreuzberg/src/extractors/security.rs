//! Security utilities for document extractors.
//!
//! This module provides validation and protection mechanisms against common attacks:
//! - ZIP bomb detection (decompression bombs)
//! - XML entity expansion limits
//! - Nesting depth limits
//! - Input size limits
//! - Entity length validation

#[cfg(feature = "archives")]
use std::io::{Read, Seek};

/// Configuration for security limits across extractors.
///
/// All limits are intentionally conservative to prevent DoS attacks
/// while still supporting legitimate documents.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SecurityLimits {
    /// Maximum uncompressed size for archives (500 MB)
    pub max_archive_size: usize,

    /// Maximum compression ratio before flagging as potential bomb (100:1)
    pub max_compression_ratio: usize,

    /// Maximum number of files in archive (10,000)
    pub max_files_in_archive: usize,

    /// Maximum nesting depth for structures (100)
    pub max_nesting_depth: usize,

    /// Maximum entity/string length (32)
    pub max_entity_length: usize,

    /// Maximum string growth per document (100 MB)
    pub max_content_size: usize,

    /// Maximum iterations per operation
    pub max_iterations: usize,

    /// Maximum XML depth (100 levels)
    pub max_xml_depth: usize,

    /// Maximum cells per table (100,000)
    pub max_table_cells: usize,
}

impl Default for SecurityLimits {
    fn default() -> Self {
        Self {
            max_archive_size: 500 * 1024 * 1024,
            max_compression_ratio: 100,
            max_files_in_archive: 10_000,
            max_nesting_depth: 100,
            max_entity_length: 32,
            max_content_size: 100 * 1024 * 1024,
            max_iterations: 10_000_000,
            max_xml_depth: 100,
            max_table_cells: 100_000,
        }
    }
}

/// Security validation errors.
#[derive(Debug, Clone)]
pub enum SecurityError {
    /// Potential ZIP bomb detected
    ZipBombDetected {
        compressed_size: u64,
        uncompressed_size: u64,
        ratio: f64,
    },

    /// Archive exceeds maximum size
    ArchiveTooLarge { size: u64, max: usize },

    /// Archive contains too many files
    TooManyFiles { count: usize, max: usize },

    /// Nesting too deep
    NestingTooDeep { depth: usize, max: usize },

    /// Content exceeds maximum size
    ContentTooLarge { size: usize, max: usize },

    /// Entity/string too long
    EntityTooLong { length: usize, max: usize },

    /// Too many iterations
    TooManyIterations { count: usize, max: usize },

    /// XML depth exceeded
    XmlDepthExceeded { depth: usize, max: usize },

    /// Too many table cells
    TooManyCells { cells: usize, max: usize },
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SecurityError::ZipBombDetected {
                compressed_size,
                uncompressed_size,
                ratio,
            } => {
                write!(
                    f,
                    "Potential ZIP bomb detected: compressed {}B -> uncompressed {}B (ratio: {:.1}:1)",
                    compressed_size, uncompressed_size, ratio
                )
            }
            SecurityError::ArchiveTooLarge { size, max } => {
                write!(f, "Archive too large: {} bytes (max: {} bytes)", size, max)
            }
            SecurityError::TooManyFiles { count, max } => {
                write!(f, "Archive has too many files: {} (max: {})", count, max)
            }
            SecurityError::NestingTooDeep { depth, max } => {
                write!(f, "Nesting too deep: {} levels (max: {})", depth, max)
            }
            SecurityError::ContentTooLarge { size, max } => {
                write!(f, "Content too large: {} bytes (max: {} bytes)", size, max)
            }
            SecurityError::EntityTooLong { length, max } => {
                write!(f, "Entity too long: {} chars (max: {})", length, max)
            }
            SecurityError::TooManyIterations { count, max } => {
                write!(f, "Too many iterations: {} (max: {})", count, max)
            }
            SecurityError::XmlDepthExceeded { depth, max } => {
                write!(f, "XML depth exceeded: {} (max: {})", depth, max)
            }
            SecurityError::TooManyCells { cells, max } => {
                write!(f, "Too many table cells: {} (max: {})", cells, max)
            }
        }
    }
}

impl std::error::Error for SecurityError {}

/// Helper struct for validating ZIP archives for security issues.
#[cfg(feature = "archives")]
pub struct ZipBombValidator {
    limits: SecurityLimits,
}

#[cfg(feature = "archives")]
impl ZipBombValidator {
    /// Create a new ZIP bomb validator.
    pub(crate) fn new(limits: SecurityLimits) -> Self {
        Self { limits }
    }

    /// Validate a ZIP archive for security issues.
    ///
    /// # Arguments
    /// * `archive` - Mutable ZIP archive to validate
    ///
    /// # Returns
    /// * `Ok(())` if archive is safe
    /// * `Err(SecurityError)` if security limit violated
    pub(crate) fn validate<R: Read + Seek>(&self, archive: &mut zip::ZipArchive<R>) -> Result<(), SecurityError> {
        let file_count = archive.len();

        if file_count > self.limits.max_files_in_archive {
            return Err(SecurityError::TooManyFiles {
                count: file_count,
                max: self.limits.max_files_in_archive,
            });
        }

        let mut total_uncompressed: u64 = 0;
        let mut total_compressed: u64 = 0;

        for i in 0..file_count {
            if let Ok(file) = archive.by_index(i) {
                let compressed_size = file.compressed_size();
                let uncompressed_size = file.size();

                total_uncompressed += uncompressed_size;
                total_compressed += compressed_size;

                if compressed_size > 0 && uncompressed_size > 0 {
                    let ratio = uncompressed_size as f64 / compressed_size as f64;
                    if ratio > self.limits.max_compression_ratio as f64 {
                        return Err(SecurityError::ZipBombDetected {
                            compressed_size,
                            uncompressed_size,
                            ratio,
                        });
                    }
                }
            }
        }

        if total_uncompressed > self.limits.max_archive_size as u64 {
            return Err(SecurityError::ArchiveTooLarge {
                size: total_uncompressed,
                max: self.limits.max_archive_size,
            });
        }

        if total_compressed > 0 {
            let ratio = total_uncompressed as f64 / total_compressed as f64;
            if ratio > self.limits.max_compression_ratio as f64 {
                return Err(SecurityError::ZipBombDetected {
                    compressed_size: total_compressed,
                    uncompressed_size: total_uncompressed,
                    ratio,
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limits() {
        let limits = SecurityLimits::default();
        assert_eq!(limits.max_archive_size, 500 * 1024 * 1024);
        assert_eq!(limits.max_nesting_depth, 100);
        assert_eq!(limits.max_entity_length, 32);
    }
}
