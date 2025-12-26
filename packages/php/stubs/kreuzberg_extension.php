<?php

declare(strict_types=1);

/**
 * Type stubs for Kreuzberg PHP extension functions.
 *
 * These functions are provided by the native Rust extension (kreuzberg.so/.dll).
 * This file provides type hints for IDEs and static analyzers.
 *
 * DO NOT include this file in your application - these functions are automatically
 * available when the extension is loaded.
 *
 * @internal
 */

/**
 * Extract content from a file (native extension function).
 *
 * @param string $filePath Path to the file
 * @param string|null $mimeType Optional MIME type hint
 * @param array<string, mixed> $config Extraction configuration array
 * @return array<string, mixed> Extraction result array
 * @throws \Exception If extraction fails
 */
function kreuzberg_extract_file(string $filePath, ?string $mimeType, array $config): array
{
}

/**
 * Extract content from bytes (native extension function).
 *
 * @param string $data File content as bytes
 * @param string $mimeType MIME type of the data
 * @param array<string, mixed> $config Extraction configuration array
 * @return array<string, mixed> Extraction result array
 * @throws \Exception If extraction fails
 */
function kreuzberg_extract_bytes(string $data, string $mimeType, array $config): array
{
}

/**
 * Extract content from multiple files in parallel (native extension function).
 *
 * @param array<string> $paths List of file paths
 * @param array<string, mixed> $config Extraction configuration array
 * @return array<array<string, mixed>> List of extraction result arrays
 * @throws \Exception If extraction fails
 */
function kreuzberg_batch_extract_files(array $paths, array $config): array
{
}

/**
 * Extract content from multiple byte arrays in parallel (native extension function).
 *
 * @param array<string> $dataList List of file contents as bytes
 * @param array<string> $mimeTypes List of MIME types
 * @param array<string, mixed> $config Extraction configuration array
 * @return array<array<string, mixed>> List of extraction result arrays
 * @throws \Exception If extraction fails
 */
function kreuzberg_batch_extract_bytes(array $dataList, array $mimeTypes, array $config): array
{
}

/**
 * Detect MIME type from file bytes (native extension function).
 *
 * @param string $data File content as bytes
 * @return string Detected MIME type
 */
function kreuzberg_detect_mime_type(string $data): string
{
}

/**
 * Detect MIME type from file path (native extension function).
 *
 * @param string $path Path to the file
 * @return string Detected MIME type
 */
function kreuzberg_detect_mime_type_from_path(string $path): string
{
}
