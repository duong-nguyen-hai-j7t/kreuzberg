<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Extracted table structure.
 *
 * @property-read array<array<string>> $cells Table cells (2D array)
 * @property-read string $markdown Table in markdown format
 * @property-read int $pageNumber Page number where table was found
 */
readonly class Table
{
    /**
     * @param array<array<string>> $cells
     */
    public function __construct(
        public array $cells,
        public string $markdown,
        public int $pageNumber,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var array<array<string>> $cells */
        $cells = $data['cells'] ?? [];

        /** @var string $markdown */
        $markdown = $data['markdown'] ?? '';

        /** @var int $pageNumber */
        $pageNumber = $data['page_number'] ?? 0;

        return new self(
            cells: $cells,
            markdown: $markdown,
            pageNumber: $pageNumber,
        );
    }
}
