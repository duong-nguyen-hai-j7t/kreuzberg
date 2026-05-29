// Hand-written adapters for batch extract functions.
// Provides proper Swift array conversion for RustVec<ExtractionResult> returns.
// These functions are in exclude_functions in alef.toml, but swift-bridge generates
// them with return types of RustVec<ExtractionResult>, which Swift cannot auto-convert to [ExtractionResult].

import Foundation
import RustBridge

/// Convert a batch of bytes to extraction results synchronously.
/// - Parameters:
///   - items: Array of bytes and MIME type pairs to extract
///   - config: Extraction configuration
/// - Returns: Array of extraction results, one per input item
public func batchExtractBytesSync(items: [BatchBytesItem], config: ExtractionConfig) throws -> [ExtractionResult] {
    let _rb_items: RustVec<BatchBytesItem> = { let v = RustVec<BatchBytesItem>(); for x in items { v.push(value: x) }; return v }()
    let rustVecResult = try RustBridge.batchExtractBytesSync(_rb_items, config)
    return (0..<rustVecResult.len()).map { rustVecResult.get(Int32($0))! }
}

/// Convert a batch of files to extraction results synchronously.
/// - Parameters:
///   - items: Array of file paths to extract
///   - config: Extraction configuration
/// - Returns: Array of extraction results, one per input file
public func batchExtractFilesSync(items: [BatchFileItem], config: ExtractionConfig) throws -> [ExtractionResult] {
    let _rb_items: RustVec<BatchFileItem> = { let v = RustVec<BatchFileItem>(); for x in items { v.push(value: x) }; return v }()
    let rustVecResult = try RustBridge.batchExtractFilesSync(_rb_items, config)
    return (0..<rustVecResult.len()).map { rustVecResult.get(Int32($0))! }
}

/// Convert a batch of bytes to extraction results asynchronously.
/// - Parameters:
///   - items: Array of bytes and MIME type pairs to extract
///   - config: Extraction configuration
/// - Returns: Array of extraction results, one per input item
public func batchExtractBytes(items: [BatchBytesItem], config: ExtractionConfig) async throws -> [ExtractionResult] {
    return try await Task.detached(priority: .userInitiated) {
        let _rb_items: RustVec<BatchBytesItem> = { let v = RustVec<BatchBytesItem>(); for x in items { v.push(value: x) }; return v }()
        let rustVecResult = try RustBridge.batchExtractBytes(_rb_items, config)
        return (0..<rustVecResult.len()).map { rustVecResult.get(Int32($0))! }
    }.value
}

/// Convert a batch of files to extraction results asynchronously.
/// - Parameters:
///   - items: Array of file paths to extract
///   - config: Extraction configuration
/// - Returns: Array of extraction results, one per input file
public func batchExtractFiles(items: [BatchFileItem], config: ExtractionConfig) async throws -> [ExtractionResult] {
    return try await Task.detached(priority: .userInitiated) {
        let _rb_items: RustVec<BatchFileItem> = { let v = RustVec<BatchFileItem>(); for x in items { v.push(value: x) }; return v }()
        let rustVecResult = try RustBridge.batchExtractFiles(_rb_items, config)
        return (0..<rustVecResult.len()).map { rustVecResult.get(Int32($0))! }
    }.value
}
