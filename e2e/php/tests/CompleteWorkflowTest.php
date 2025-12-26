<?php

declare(strict_types=1);

namespace Kreuzberg\E2E\Tests;

use Kreuzberg\Config\ChunkingConfig;
use Kreuzberg\Config\EmbeddingConfig;
use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\OcrConfig;
use Kreuzberg\Config\PageConfig;
use Kreuzberg\Kreuzberg;
use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\Attributes\RequiresPhpExtension;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;

/**
 * End-to-end tests for complete document processing workflows.
 *
 * Tests real-world scenarios combining multiple features.
 */
#[Group('e2e')]
#[RequiresPhpExtension('kreuzberg')]
final class CompleteWorkflowTest extends TestCase
{
    private string $testDocumentsPath;

    protected function setUp(): void
    {
        if (!extension_loaded('kreuzberg')) {
            $this->markTestSkipped('Kreuzberg extension is not loaded');
        }

        $this->testDocumentsPath = dirname(__DIR__, 3) . '/test_documents';

        if (!is_dir($this->testDocumentsPath)) {
            $this->markTestSkipped('test_documents directory not found');
        }
    }

    #[Test]
    public function it_processes_pdf_with_full_extraction_pipeline(): void
    {
        $filePath = $this->testDocumentsPath . '/pdfs/code_and_formula.pdf';

        if (!file_exists($filePath)) {
            $this->markTestSkipped("Test file not found: {$filePath}");
        }

        $config = new ExtractionConfig(
            extractImages: true,
            extractTables: true,
            preserveFormatting: true,
            page: new PageConfig(
                extractPages: true,
                insertPageMarkers: true,
                markerFormat: '--- Page {page_number} ---'
            ),
            chunking: new ChunkingConfig(
                maxChunkSize: 500,
                chunkOverlap: 50,
                respectSentences: true
            )
        );

        $kreuzberg = new Kreuzberg($config);
        $result = $kreuzberg->extractFile($filePath);

        // Verify all extraction features worked
        $this->assertNotEmpty($result->content, 'Content should be extracted');
        $this->assertSame('application/pdf', $result->mimeType);
        $this->assertGreaterThan(0, $result->metadata->pageCount);
        $this->assertIsArray($result->tables);
        $this->assertNotNull($result->pages, 'Pages should be extracted');
        $this->assertNotNull($result->chunks, 'Chunks should be created');
    }

    #[Test]
    public function it_performs_complete_ocr_workflow_on_image(): void
    {
        $imagePath = $this->testDocumentsPath . '/images/invoice_image.png';

        if (!file_exists($imagePath)) {
            $this->markTestSkipped("Test file not found: {$imagePath}");
        }

        $config = new ExtractionConfig(
            ocr: new OcrConfig(
                backend: 'tesseract',
                language: 'eng'
            ),
            extractTables: true,
            chunking: new ChunkingConfig(
                maxChunkSize: 300,
                chunkOverlap: 30
            )
        );

        $kreuzberg = new Kreuzberg($config);
        $result = $kreuzberg->extractFile($imagePath);

        // Verify OCR pipeline
        $this->assertNotNull($result->content, 'OCR should extract text');
        $this->assertStringContainsString('image/', strtolower($result->mimeType));
        $this->assertTrue(mb_check_encoding($result->content, 'UTF-8'),
            'OCR output should be UTF-8');
    }

    #[Test]
    public function it_processes_batch_of_mixed_documents(): void
    {
        $files = [
            $this->testDocumentsPath . '/pdfs/code_and_formula.pdf',
            $this->testDocumentsPath . '/extraction_test.md',
        ];

        if (file_exists($this->testDocumentsPath . '/extraction_test.odt')) {
            $files[] = $this->testDocumentsPath . '/extraction_test.odt';
        }

        foreach ($files as $file) {
            if (!file_exists($file)) {
                $this->markTestSkipped("Test file not found: {$file}");
            }
        }

        $config = new ExtractionConfig(
            extractTables: true,
            chunking: new ChunkingConfig(
                maxChunkSize: 400,
                chunkOverlap: 40
            )
        );

        $kreuzberg = new Kreuzberg($config);
        $results = $kreuzberg->batchExtractFiles($files);

        // Verify batch processing
        $this->assertCount(count($files), $results);

        foreach ($results as $index => $result) {
            $this->assertNotEmpty($result->content,
                "Document {$index} should have content");
            $this->assertNotEmpty($result->mimeType,
                "Document {$index} should have MIME type");
            $this->assertIsArray($result->tables,
                "Document {$index} should have tables array");
        }
    }

    #[Test]
    public function it_extracts_document_with_embeddings_workflow(): void
    {
        $filePath = $this->testDocumentsPath . '/extraction_test.md';

        if (!file_exists($filePath)) {
            $this->markTestSkipped("Test file not found: {$filePath}");
        }

        $config = new ExtractionConfig(
            chunking: new ChunkingConfig(
                maxChunkSize: 500,
                chunkOverlap: 50,
                respectSentences: true
            ),
            embedding: new EmbeddingConfig(
                model: 'all-minilm-l6-v2',
                normalize: true
            )
        );

        $kreuzberg = new Kreuzberg($config);
        $result = $kreuzberg->extractFile($filePath);

        // Verify embeddings workflow
        $this->assertNotEmpty($result->content);
        $this->assertNotNull($result->chunks, 'Chunks should be created');

        if (!empty($result->chunks)) {
            $chunk = $result->chunks[0];
            $this->assertNotEmpty($chunk->content);

            if ($chunk->embedding !== null) {
                $this->assertIsArray($chunk->embedding,
                    'Embeddings should be generated');
                $this->assertNotEmpty($chunk->embedding);
            }
        }
    }

    #[Test]
    public function it_extracts_tables_and_converts_to_markdown(): void
    {
        $pdfFiles = glob($this->testDocumentsPath . '/pdfs_with_tables/*.pdf');

        if (empty($pdfFiles)) {
            $this->markTestSkipped('No PDF files with tables found');
        }

        $config = new ExtractionConfig(extractTables: true);
        $kreuzberg = new Kreuzberg($config);
        $result = $kreuzberg->extractFile($pdfFiles[0]);

        // Verify table extraction and markdown conversion
        $this->assertNotEmpty($result->content);

        if (!empty($result->tables)) {
            $table = $result->tables[0];

            $this->assertIsString($table->markdown,
                'Table should have markdown representation');
            $this->assertStringContainsString('|', $table->markdown,
                'Markdown should have table syntax');
            $this->assertIsInt($table->pageNumber);
            $this->assertIsArray($table->data);
        }
    }

    #[Test]
    public function it_processes_multipage_document_with_page_markers(): void
    {
        $pdfPath = $this->testDocumentsPath . '/pdfs/code_and_formula.pdf';

        if (!file_exists($pdfPath)) {
            $this->markTestSkipped("Test file not found: {$pdfPath}");
        }

        $config = new ExtractionConfig(
            page: new PageConfig(
                extractPages: true,
                insertPageMarkers: true,
                markerFormat: '=== PAGE {page_number} ==='
            )
        );

        $kreuzberg = new Kreuzberg($config);
        $result = $kreuzberg->extractFile($pdfPath);

        // Verify page extraction
        $this->assertNotEmpty($result->content);

        if ($result->metadata->pageCount > 1) {
            $this->assertNotNull($result->pages,
                'Multipage document should have pages array');

            if (!empty($result->pages)) {
                $this->assertIsArray($result->pages);
                $this->assertGreaterThan(0, count($result->pages));
            }
        }
    }

    #[Test]
    public function it_handles_bytes_extraction_workflow(): void
    {
        $filePath = $this->testDocumentsPath . '/pdfs/code_and_formula.pdf';

        if (!file_exists($filePath)) {
            $this->markTestSkipped("Test file not found: {$filePath}");
        }

        $bytes = file_get_contents($filePath);

        $config = new ExtractionConfig(
            extractTables: true,
            chunking: new ChunkingConfig(
                maxChunkSize: 400,
                chunkOverlap: 40
            )
        );

        $kreuzberg = new Kreuzberg($config);
        $result = $kreuzberg->extractBytes($bytes, 'application/pdf');

        // Verify bytes extraction workflow
        $this->assertNotEmpty($result->content);
        $this->assertSame('application/pdf', $result->mimeType);
        $this->assertIsArray($result->tables);
        $this->assertNotNull($result->chunks);
    }

    #[Test]
    public function it_processes_office_documents_end_to_end(): void
    {
        $filePath = $this->testDocumentsPath . '/office/document.docx';

        if (!file_exists($filePath)) {
            $filePath = $this->testDocumentsPath . '/extraction_test.docx';

            if (!file_exists($filePath)) {
                $this->markTestSkipped('No DOCX file found for testing');
            }
        }

        $config = new ExtractionConfig(
            extractTables: true,
            extractImages: true,
            preserveFormatting: true
        );

        $kreuzberg = new Kreuzberg($config);
        $result = $kreuzberg->extractFile($filePath);

        // Verify Office document processing
        $this->assertNotEmpty($result->content);
        $this->assertStringContainsString('application/', $result->mimeType);
        $this->assertNotNull($result->metadata);
        $this->assertIsArray($result->tables);
    }

    #[Test]
    public function it_validates_complete_extraction_result_structure(): void
    {
        $filePath = $this->testDocumentsPath . '/pdfs/code_and_formula.pdf';

        if (!file_exists($filePath)) {
            $this->markTestSkipped("Test file not found: {$filePath}");
        }

        $config = new ExtractionConfig(
            extractImages: true,
            extractTables: true,
            page: new PageConfig(extractPages: true),
            chunking: new ChunkingConfig(maxChunkSize: 500)
        );

        $kreuzberg = new Kreuzberg($config);
        $result = $kreuzberg->extractFile($filePath);

        // Validate complete result structure
        $this->assertObjectHasProperty('content', $result);
        $this->assertObjectHasProperty('mimeType', $result);
        $this->assertObjectHasProperty('metadata', $result);
        $this->assertObjectHasProperty('tables', $result);
        $this->assertObjectHasProperty('chunks', $result);
        $this->assertObjectHasProperty('images', $result);
        $this->assertObjectHasProperty('pages', $result);

        // Validate metadata structure
        $this->assertObjectHasProperty('pageCount', $result->metadata);
        $this->assertObjectHasProperty('title', $result->metadata);
        $this->assertObjectHasProperty('author', $result->metadata);

        // Validate data types
        $this->assertIsString($result->content);
        $this->assertIsString($result->mimeType);
        $this->assertIsArray($result->tables);
    }

    #[Test]
    public function it_processes_multiple_formats_with_consistent_api(): void
    {
        $testCases = [
            'pdf' => $this->testDocumentsPath . '/pdfs/code_and_formula.pdf',
            'markdown' => $this->testDocumentsPath . '/extraction_test.md',
            'odt' => $this->testDocumentsPath . '/extraction_test.odt',
        ];

        $config = new ExtractionConfig(extractTables: true);
        $kreuzberg = new Kreuzberg($config);

        foreach ($testCases as $format => $filePath) {
            if (!file_exists($filePath)) {
                continue;
            }

            $result = $kreuzberg->extractFile($filePath);

            // Verify consistent API across formats
            $this->assertNotEmpty($result->content,
                "{$format} should have content");
            $this->assertNotEmpty($result->mimeType,
                "{$format} should have MIME type");
            $this->assertNotNull($result->metadata,
                "{$format} should have metadata");
            $this->assertIsArray($result->tables,
                "{$format} should have tables array");
        }
    }
}
