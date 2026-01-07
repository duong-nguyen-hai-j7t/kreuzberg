package main

import (
	"kreuzberg"
)

func main() {
	// Example 1: Preset model (recommended)
	// Fast, balanced, or quality preset configurations optimized for common use cases.
	embeddingConfig := kreuzberg.EmbeddingConfig{
		Model: kreuzberg.EmbeddingModelType{
			Type: "preset",
			Name: "balanced",
		},
		BatchSize:            32,
		Normalize:            true,
		ShowDownloadProgress: true,
		CacheDir:             "~/.cache/kreuzberg/embeddings",
	}

	// Available presets:
	// - "fast" (384 dims): Quick prototyping, development, resource-constrained
	// - "balanced" (768 dims): Production, general-purpose RAG, English documents
	// - "quality" (1024 dims): Complex documents, maximum accuracy
	// - "multilingual" (768 dims): International documents, 100+ languages

	// Example 2: FastEmbed model (requires embeddings feature)
	// Direct access to specific fastembed models with custom dimensions.
	embeddingConfig = kreuzberg.EmbeddingConfig{
		Model: kreuzberg.EmbeddingModelType{
			Type:       "fastembed",
			Model:      "BAAI/bge-small-en-v1.5",
			Dimensions: 384,
		},
		BatchSize:            32,
		Normalize:            true,
		ShowDownloadProgress: true,
		CacheDir:             "",  // Uses default: .kreuzberg/embeddings/
	}

	// Supported FastEmbed models:
	// - "AllMiniLML6V2Q" (384 dims): Quantized, fastest
	// - "BGEBaseENV15" (768 dims): Balanced quality/speed
	// - "BGELargeENV15" (1024 dims): High quality, slower
	// - "MultilingualE5Base" (768 dims): Multilingual support

	// Example 3: Custom HuggingFace model
	// For advanced users wanting specific HuggingFace embedding models.
	embeddingConfig = kreuzberg.EmbeddingConfig{
		Model: kreuzberg.EmbeddingModelType{
			Type:       "custom",
			ModelID:    "sentence-transformers/all-mpnet-base-v2",
			Dimensions: 768,
		},
		BatchSize:            16,  // Larger model requires smaller batch size
		Normalize:            true,
		ShowDownloadProgress: true,
		CacheDir:             "/var/cache/embeddings",
	}

	// Integration with ChunkingConfig
	// Add embeddings to your chunking configuration:
	chunkingConfig := kreuzberg.ChunkingConfig{
		MaxChars:  1024,
		MaxOverlap: 100,
		Preset:     "balanced",
		Embedding: &kreuzberg.EmbeddingConfig{
			Model: kreuzberg.EmbeddingModelType{
				Type: "preset",
				Name: "balanced",
			},
			BatchSize: 32,
			Normalize: true,
		},
	}

	extractionConfig := kreuzberg.ExtractionConfig{
		Chunking: &chunkingConfig,
	}

	_ = embeddingConfig
	_ = extractionConfig
}

// Key parameter explanations:
//
// BatchSize: Number of texts to embed at once (32-128 typical)
//   - Larger batches are faster but use more memory
//   - Smaller batches for resource-constrained environments
//
// Normalize: Whether to normalize vectors (L2 norm)
//   - true (recommended): Enables cosine similarity in vector DBs
//   - false: Raw embedding values
//
// CacheDir: Where to store downloaded models
//   - Empty string: Uses .kreuzberg/embeddings/ in current directory
//   - Non-empty: Custom directory for model storage
//
// ShowDownloadProgress: Display download progress bar
//   - Useful for monitoring large model downloads
