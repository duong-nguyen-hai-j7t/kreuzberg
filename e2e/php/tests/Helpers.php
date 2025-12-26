<?php

declare(strict_types=1);

namespace Kreuzberg\E2E\Tests;

use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Types\ExtractionResult;
use PHPUnit\Framework\Assert;

/**
 * Helper functions for E2E tests.
 *
 * Provides common assertions and utilities for testing document extraction.
 */
final class Helpers
{
    /**
     * Resolve document path relative to test_documents directory.
     */
    public static function resolveDocument(string $relativePath): string
    {
        $testDocumentsPath = dirname(__DIR__, 3) . '/test_documents';

        return $testDocumentsPath . '/' . $relativePath;
    }

    /**
     * Build extraction config from test configuration array.
     *
     * @param array<string, mixed>|null $configData
     */
    public static function buildConfig(?array $configData): ?ExtractionConfig
    {
        if ($configData === null) {
            return null;
        }

        return new ExtractionConfig(
            extractImages: $configData['extract_images'] ?? false,
            extractTables: $configData['extract_tables'] ?? true,
            preserveFormatting: $configData['preserve_formatting'] ?? false,
            outputFormat: $configData['output_format'] ?? null,
        );
    }

    /**
     * Assert MIME type matches expected values.
     *
     * @param array<string> $expectedMimeTypes
     */
    public static function assertExpectedMime(ExtractionResult $result, array $expectedMimeTypes): void
    {
        $mimeType = strtolower($result->mimeType);
        $found = false;

        foreach ($expectedMimeTypes as $expected) {
            if (str_contains($mimeType, strtolower($expected))) {
                $found = true;
                break;
            }
        }

        Assert::assertTrue($found,
            "MIME type '{$result->mimeType}' does not match any of: " . implode(', ', $expectedMimeTypes));
    }

    /**
     * Assert minimum content length.
     */
    public static function assertMinContentLength(ExtractionResult $result, int $minLength): void
    {
        $actualLength = strlen($result->content);

        Assert::assertGreaterThanOrEqual($minLength, $actualLength,
            "Content length {$actualLength} is less than minimum {$minLength}");
    }

    /**
     * Assert maximum content length.
     */
    public static function assertMaxContentLength(ExtractionResult $result, int $maxLength): void
    {
        $actualLength = strlen($result->content);

        Assert::assertLessThanOrEqual($maxLength, $actualLength,
            "Content length {$actualLength} exceeds maximum {$maxLength}");
    }

    /**
     * Assert content contains any of the provided snippets.
     *
     * @param array<string> $snippets
     */
    public static function assertContentContainsAny(ExtractionResult $result, array $snippets): void
    {
        $content = $result->content;
        $found = false;

        foreach ($snippets as $snippet) {
            if (str_contains($content, $snippet)) {
                $found = true;
                break;
            }
        }

        Assert::assertTrue($found,
            'Content does not contain any of the expected snippets: ' . implode(', ', $snippets));
    }

    /**
     * Assert content contains all of the provided snippets.
     *
     * @param array<string> $snippets
     */
    public static function assertContentContainsAll(ExtractionResult $result, array $snippets): void
    {
        $content = $result->content;

        foreach ($snippets as $snippet) {
            Assert::assertStringContainsString($snippet, $content,
                "Content does not contain expected snippet: '{$snippet}'");
        }
    }

    /**
     * Assert table count matches expected value.
     */
    public static function assertTableCount(ExtractionResult $result, int $expectedCount): void
    {
        $actualCount = count($result->tables);

        Assert::assertSame($expectedCount, $actualCount,
            "Expected {$expectedCount} tables, found {$actualCount}");
    }

    /**
     * Assert minimum table count.
     */
    public static function assertMinTableCount(ExtractionResult $result, int $minCount): void
    {
        $actualCount = count($result->tables);

        Assert::assertGreaterThanOrEqual($minCount, $actualCount,
            "Expected at least {$minCount} tables, found {$actualCount}");
    }

    /**
     * Assert detected languages match expected values.
     *
     * @param array<string> $expectedLanguages
     */
    public static function assertDetectedLanguages(ExtractionResult $result, array $expectedLanguages): void
    {
        Assert::assertNotNull($result->detectedLanguages,
            'Language detection should be enabled');

        Assert::assertIsArray($result->detectedLanguages,
            'Detected languages should be an array');

        foreach ($expectedLanguages as $lang) {
            Assert::assertContains($lang, $result->detectedLanguages,
                "Expected language '{$lang}' not detected");
        }
    }

    /**
     * Assert metadata field matches expected value.
     *
     * @param mixed $expectedValue
     */
    public static function assertMetadataExpectation(ExtractionResult $result, string $field, mixed $expectedValue): void
    {
        Assert::assertNotNull($result->metadata,
            'Metadata should be present');

        Assert::assertObjectHasProperty($field, $result->metadata,
            "Metadata should have field '{$field}'");

        $actualValue = $result->metadata->{$field};

        Assert::assertSame($expectedValue, $actualValue,
            "Metadata field '{$field}' expected '{$expectedValue}', got '{$actualValue}'");
    }

    /**
     * Assert page count matches expected value.
     */
    public static function assertPageCount(ExtractionResult $result, int $expectedCount): void
    {
        Assert::assertNotNull($result->metadata,
            'Metadata should be present');

        Assert::assertSame($expectedCount, $result->metadata->pageCount,
            "Expected {$expectedCount} pages, found {$result->metadata->pageCount}");
    }

    /**
     * Assert minimum page count.
     */
    public static function assertMinPageCount(ExtractionResult $result, int $minCount): void
    {
        Assert::assertNotNull($result->metadata,
            'Metadata should be present');

        Assert::assertGreaterThanOrEqual($minCount, $result->metadata->pageCount,
            "Expected at least {$minCount} pages, found {$result->metadata->pageCount}");
    }

    /**
     * Assert chunks were generated.
     */
    public static function assertHasChunks(ExtractionResult $result): void
    {
        Assert::assertNotNull($result->chunks,
            'Chunks should be generated when chunking config is provided');

        Assert::assertIsArray($result->chunks,
            'Chunks should be an array');

        Assert::assertNotEmpty($result->chunks,
            'Chunks array should not be empty');
    }

    /**
     * Assert minimum chunk count.
     */
    public static function assertMinChunkCount(ExtractionResult $result, int $minCount): void
    {
        self::assertHasChunks($result);

        Assert::assertGreaterThanOrEqual($minCount, count($result->chunks),
            "Expected at least {$minCount} chunks");
    }

    /**
     * Assert images were extracted.
     */
    public static function assertHasImages(ExtractionResult $result): void
    {
        Assert::assertNotNull($result->images,
            'Images should be extracted when extractImages is enabled');

        Assert::assertIsArray($result->images,
            'Images should be an array');

        Assert::assertNotEmpty($result->images,
            'Images array should not be empty');
    }

    /**
     * Assert content is valid UTF-8.
     */
    public static function assertValidUtf8(ExtractionResult $result): void
    {
        Assert::assertTrue(mb_check_encoding($result->content, 'UTF-8'),
            'Content should be valid UTF-8');
    }

    /**
     * Assert content is not empty.
     */
    public static function assertNotEmptyContent(ExtractionResult $result): void
    {
        Assert::assertNotEmpty($result->content,
            'Content should not be empty');
    }
}
