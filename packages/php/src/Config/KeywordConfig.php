<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Keyword extraction configuration.
 */
readonly class KeywordConfig
{
    public function __construct(
        public int $maxKeywords = 10,
        public float $minScore = 0.0,
        public ?string $language = 'en',
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'max_keywords' => $this->maxKeywords,
            'min_score' => $this->minScore,
            'language' => $this->language,
        ], static fn ($value): bool => $value !== null);
    }
}
