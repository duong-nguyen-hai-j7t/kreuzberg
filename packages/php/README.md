# Kreuzberg PHP

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/badge/docs-kreuzberg.dev-blue)](https://kreuzberg.dev/)

High-performance document intelligence for PHP. Extract text, metadata, and structured information from PDFs, Office documents, images, and 56 formats.

**Powered by a Rust core** – Native performance for document extraction.

> **🚀 Version 4.0.0 Release Candidate**
> This is a pre-release version. We invite you to test the library and [report any issues](https://github.com/kreuzberg-dev/kreuzberg/issues) you encounter.

## Features

- **56+ File Formats**: PDF, DOCX, XLSX, PPTX, images, HTML, XML, email, archives, and more
- **OCR Support**: Tesseract integration for scanned documents and images
- **Table Extraction**: Extract structured tables from PDFs and documents
- **Metadata Extraction**: Rich metadata for all supported formats
- **High Performance**: 10-50x faster than pure PHP solutions (Rust core)
- **Batch Processing**: Process multiple documents in parallel
- **Text Chunking**: Intelligent text segmentation for RAG applications
- **Embeddings**: Generate vector embeddings for semantic search
- **Type Safe**: Full PHP 8.2+ type hints and readonly classes

## System Requirements

- PHP 8.2 or higher
- Kreuzberg PHP extension (kreuzberg.so/.dll)
- Tesseract OCR (optional, for OCR functionality)
- ONNX Runtime (optional, for embeddings)

### Installing Tesseract

```bash
# macOS
brew install tesseract

# Ubuntu/Debian
sudo apt install tesseract-ocr

# Windows
# Download from: https://github.com/UB-Mannheim/tesseract/wiki
```

### Installing ONNX Runtime

```bash
# macOS
brew install onnxruntime

# Ubuntu/Debian
sudo apt install libonnxruntime libonnxruntime-dev

# Windows (MSVC)
scoop install onnxruntime
# OR download from https://github.com/microsoft/onnxruntime/releases
```

## Installation

```bash
composer require kreuzberg/kreuzberg
```

The PHP extension (kreuzberg.so/.dll) must be installed separately. Download the appropriate extension for your platform from the [releases page](https://github.com/kreuzberg-dev/kreuzberg/releases).

Add to your `php.ini`:

```ini
extension=kreuzberg.so  ; Linux/macOS
; or
extension=kreuzberg.dll  ; Windows
```

## Quick Start

### Simple Extraction

```php
<?php

use Kreuzberg\Kreuzberg;

$kreuzberg = new Kreuzberg();
$result = $kreuzberg->extractFile('document.pdf');

echo $result->content;
print_r($result->metadata);
print_r($result->tables);
```

### Procedural API

```php
<?php

use function Kreuzberg\extract_file;

$result = extract_file('document.pdf');
echo $result->content;
```

### Batch Processing

```php
<?php

use Kreuzberg\Kreuzberg;

$kreuzberg = new Kreuzberg();
$files = ['doc1.pdf', 'doc2.docx', 'doc3.xlsx'];
$results = $kreuzberg->batchExtractFiles($files);

foreach ($results as $result) {
    echo $result->content . "\n";
}
```

## OCR Support

### Basic OCR with Tesseract

```php
<?php

use Kreuzberg\Kreuzberg;
use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\OcrConfig;

$config = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'tesseract',
        language: 'eng'
    )
);

$kreuzberg = new Kreuzberg($config);
$result = $kreuzberg->extractFile('scanned.pdf');
```

### Advanced OCR Configuration

```php
<?php

use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\OcrConfig;
use Kreuzberg\Config\TesseractConfig;
use Kreuzberg\Config\ImagePreprocessingConfig;
use function Kreuzberg\extract_file;

$config = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'tesseract',
        language: 'eng',
        tesseractConfig: new TesseractConfig(
            psm: 6,
            enableTableDetection: true,
            tesseditCharWhitelist: '0123456789'
        ),
        imagePreprocessing: new ImagePreprocessingConfig(
            targetDpi: 300,
            denoise: true,
            sharpen: true
        )
    )
);

$result = extract_file('invoice.pdf', config: $config);
```

## Table Extraction

```php
<?php

use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\OcrConfig;
use Kreuzberg\Config\TesseractConfig;
use function Kreuzberg\extract_file;

$config = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'tesseract',
        tesseractConfig: new TesseractConfig(
            enableTableDetection: true
        )
    )
);

$result = extract_file('financial_report.pdf', config: $config);

foreach ($result->tables as $table) {
    echo "Table on page {$table->pageNumber}:\n";
    echo $table->markdown . "\n\n";

    // Or access raw cells
    foreach ($table->cells as $row) {
        foreach ($row as $cell) {
            echo $cell . "\t";
        }
        echo "\n";
    }
}
```

## Text Chunking & Embeddings

```php
<?php

use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\ChunkingConfig;
use Kreuzberg\Config\EmbeddingConfig;
use function Kreuzberg\extract_file;

$config = new ExtractionConfig(
    chunking: new ChunkingConfig(
        maxChunkSize: 512,
        chunkOverlap: 50,
        respectSentences: true
    ),
    embedding: new EmbeddingConfig(
        model: 'all-MiniLM-L6-v2',
        normalize: true
    )
);

$result = extract_file('long_document.pdf', config: $config);

foreach ($result->chunks as $chunk) {
    echo "Chunk {$chunk->metadata->chunkIndex}:\n";
    echo $chunk->content . "\n";

    if ($chunk->embedding !== null) {
        echo "Embedding dimension: " . count($chunk->embedding) . "\n";
    }
}
```

## Image Extraction

```php
<?php

use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\ImageExtractionConfig;
use Kreuzberg\Config\OcrConfig;
use function Kreuzberg\extract_file;

$config = new ExtractionConfig(
    imageExtraction: new ImageExtractionConfig(
        extractImages: true,
        performOcr: true,  // OCR on extracted images
        minWidth: 100,
        minHeight: 100
    ),
    ocr: new OcrConfig(backend: 'tesseract', language: 'eng')
);

$result = extract_file('presentation.pptx', config: $config);

foreach ($result->images as $image) {
    echo "Image {$image->imageIndex} from page {$image->pageNumber}\n";
    echo "Format: {$image->format}, Size: {$image->width}x{$image->height}\n";

    // Save image
    file_put_contents("image_{$image->imageIndex}.{$image->format}", $image->data);

    // Access OCR result if available
    if ($image->ocrResult !== null) {
        echo "OCR Text: {$image->ocrResult->content}\n";
    }
}
```

## Page Extraction

```php
<?php

use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\PageConfig;
use function Kreuzberg\extract_file;

$config = new ExtractionConfig(
    page: new PageConfig(
        extractPages: true,
        insertPageMarkers: true,
        markerFormat: '--- Page {page_number} ---'
    )
);

$result = extract_file('report.pdf', config: $config);

foreach ($result->pages as $page) {
    echo "=== Page {$page->pageNumber} ===\n";
    echo $page->content . "\n";

    echo "Tables: " . count($page->tables) . "\n";
    echo "Images: " . count($page->images) . "\n";
}
```

## Language Detection

```php
<?php

use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\LanguageDetectionConfig;
use function Kreuzberg\extract_file;

$config = new ExtractionConfig(
    languageDetection: new LanguageDetectionConfig(
        enabled: true,
        maxLanguages: 3,
        confidenceThreshold: 0.8
    )
);

$result = extract_file('multilingual.pdf', config: $config);

if ($result->detectedLanguages !== null) {
    echo "Detected languages: " . implode(', ', $result->detectedLanguages) . "\n";
}
```

## Keyword Extraction

```php
<?php

use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\KeywordConfig;
use function Kreuzberg\extract_file;

$config = new ExtractionConfig(
    keyword: new KeywordConfig(
        maxKeywords: 10,
        minScore: 0.0,
        language: 'en'
    )
);

$result = extract_file('article.pdf', config: $config);

// Keywords are in metadata
if ($result->metadata->keywords !== null) {
    echo "Keywords: " . implode(', ', $result->metadata->keywords) . "\n";
}
```

## Supported Formats

| Format | Extension | MIME Type | Notes |
|--------|-----------|-----------|-------|
| PDF | .pdf | application/pdf | Full support with OCR fallback |
| Word | .docx, .doc | application/vnd.openxmlformats-officedocument.wordprocessingml.document | Text, tables, images |
| Excel | .xlsx, .xls | application/vnd.openxmlformats-officedocument.spreadsheetml.sheet | Multiple sheets |
| PowerPoint | .pptx, .ppt | application/vnd.openxmlformats-officedocument.presentationml.presentation | Slides, notes |
| Images | .png, .jpg, .jpeg, .tiff, .bmp, .webp | image/* | OCR support |
| HTML | .html, .htm | text/html | Metadata extraction |
| Markdown | .md | text/markdown | Preserves structure |
| Email | .eml, .msg | message/rfc822 | Attachments, headers |
| Archives | .zip, .tar, .7z | application/zip | File listing |
| XML | .xml | text/xml | Structure analysis |
| CSV | .csv | text/csv | Delimiter detection |
| JSON | .json | application/json | Schema extraction |

...and 40+ more formats.

## API Reference

### Main Classes

- **`Kreuzberg`**: Main OOP API class
- **`ExtractionResult`**: Extraction result with content, metadata, tables
- **`Metadata`**: Document metadata (title, author, dates, etc.)
- **`Table`**: Extracted table structure
- **`Chunk`**: Text chunk with embedding
- **`ExtractedImage`**: Extracted image with optional OCR

### Configuration Classes

- **`ExtractionConfig`**: Main configuration
- **`OcrConfig`**: OCR settings
- **`TesseractConfig`**: Tesseract-specific settings
- **`ImagePreprocessingConfig`**: Image preprocessing options
- **`PdfConfig`**: PDF extraction settings
- **`ChunkingConfig`**: Text chunking settings
- **`EmbeddingConfig`**: Embedding generation settings
- **`ImageExtractionConfig`**: Image extraction settings
- **`PageConfig`**: Page extraction settings
- **`LanguageDetectionConfig`**: Language detection settings
- **`KeywordConfig`**: Keyword extraction settings

### Procedural Functions

```php
// Extraction
extract_file(string $filePath, ?string $mimeType = null, ?ExtractionConfig $config = null): ExtractionResult
extract_bytes(string $data, string $mimeType, ?ExtractionConfig $config = null): ExtractionResult
batch_extract_files(array $paths, ?ExtractionConfig $config = null): array
batch_extract_bytes(array $dataList, array $mimeTypes, ?ExtractionConfig $config = null): array

// Utilities
detect_mime_type(string $data): string
detect_mime_type_from_path(string $path): string
```

## Error Handling

```php
<?php

use Kreuzberg\Exceptions\KreuzbergException;
use function Kreuzberg\extract_file;

try {
    $result = extract_file('document.pdf');
    echo $result->content;
} catch (KreuzbergException $e) {
    echo "Extraction failed: {$e->getMessage()}\n";
    echo "Error code: {$e->getCode()}\n";
}
```

## Performance Tips

1. **Use batch processing** for multiple files
2. **Disable unnecessary features** (OCR, embeddings) if not needed
3. **Set appropriate chunk sizes** for your use case
4. **Use page extraction** only when you need per-page content
5. **Limit image extraction** with min width/height filters

## Development

### Running Tests

```bash
composer test
```

### Code Quality

```bash
# PHPStan analysis
composer lint

# Code formatting
composer format

# Check formatting
composer format:check
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Links

- **Documentation**: https://kreuzberg.dev
- **GitHub**: https://github.com/kreuzberg-dev/kreuzberg
- **Issues**: https://github.com/kreuzberg-dev/kreuzberg/issues
- **Discord**: https://discord.gg/pXxagNK2zN

## Credits

Created by [Na'aman Hirschfeld](https://github.com/Goldziher)
