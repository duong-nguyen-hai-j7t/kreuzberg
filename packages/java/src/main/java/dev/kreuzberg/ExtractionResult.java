package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

/**
 * Result of a document extraction operation.
 *
 * <p>
 * Includes extracted content, tables, metadata, detected languages, text
 * chunks, images, page structure information, and Djot content.
 */
public final class ExtractionResult {
	private final String content;
	private final String mimeType;
	private final Map<String, Object> metadata;
	private final List<Table> tables;
	private final List<String> detectedLanguages;
	private final List<Chunk> chunks;
	private final List<ExtractedImage> images;
	private final List<PageContent> pages;
	private final PageStructure pageStructure;
	private final List<Element> elements;
	@JsonProperty("djot_content")
	private final DjotContent djotContent;

	ExtractionResult(String content, String mimeType, Map<String, Object> metadata, List<Table> tables,
			List<String> detectedLanguages, List<Chunk> chunks, List<ExtractedImage> images, List<PageContent> pages,
			PageStructure pageStructure, List<Element> elements, DjotContent djotContent) {
		this.content = Objects.requireNonNull(content, "content must not be null");
		this.mimeType = Objects.requireNonNull(mimeType, "mimeType must not be null");
		this.metadata = Collections.unmodifiableMap(metadata != null ? metadata : Collections.emptyMap());
		this.tables = Collections.unmodifiableList(tables != null ? tables : Collections.emptyList());
		if (detectedLanguages != null) {
			this.detectedLanguages = Collections.unmodifiableList(detectedLanguages);
		} else {
			this.detectedLanguages = List.of();
		}
		this.chunks = Collections.unmodifiableList(chunks != null ? chunks : List.of());
		this.images = Collections.unmodifiableList(images != null ? images : List.of());
		this.pages = Collections.unmodifiableList(pages != null ? pages : List.of());
		this.pageStructure = pageStructure;
		this.elements = Collections.unmodifiableList(elements != null ? elements : List.of());
		this.djotContent = djotContent;
	}

	public String getContent() {
		return content;
	}

	public String getMimeType() {
		return mimeType;
	}

	public Map<String, Object> getMetadata() {
		return metadata;
	}

	public List<Table> getTables() {
		return tables;
	}

	public List<String> getDetectedLanguages() {
		return detectedLanguages;
	}

	public List<Chunk> getChunks() {
		return chunks;
	}

	public List<ExtractedImage> getImages() {
		return images;
	}

	/**
	 * Get the per-page content when page extraction is enabled.
	 *
	 * @return unmodifiable list of page contents (never null, but may be empty)
	 * @since 4.2.4
	 */
	public List<PageContent> getPages() {
		return pages;
	}

	/**
	 * Get the semantic elements extracted from the document.
	 *
	 * <p>
	 * Available when extraction is configured with
	 * {@code output_format="element_based"}. Returns an empty list if element
	 * extraction is not enabled or if no elements were extracted.
	 *
	 * @return unmodifiable list of semantic elements (never null, but may be empty)
	 * @since 4.1.0
	 */
	public List<Element> getElements() {
		return elements;
	}

	/**
	 * Get the page structure information (optional).
	 *
	 * <p>
	 * Available when page tracking is enabled in the extraction configuration.
	 *
	 * @return page structure, or empty if not available
	 */
	public Optional<PageStructure> getPageStructure() {
		return Optional.ofNullable(pageStructure);
	}

	public Optional<DjotContent> getDjotContent() {
		return Optional.ofNullable(djotContent);
	}

	/**
	 * Check if the extraction was successful.
	 *
	 * <p>
	 * This method always returns true for a valid ExtractionResult. If extraction
	 * fails, an exception is thrown instead of returning an unsuccessful result.
	 *
	 * @return true (always, since invalid results throw exceptions)
	 * @deprecated This method is deprecated as extraction failures now throw
	 *             exceptions. All ExtractionResult instances represent successful
	 *             extractions.
	 */
	@Deprecated(since = "0.8.0", forRemoval = true)
	public boolean isSuccess() {
		return true;
	}

	/**
	 * Get the detected language from metadata.
	 *
	 * <p>
	 * Use {@link #getDetectedLanguage()} instead, which retrieves the primary
	 * detected language from either metadata or the detectedLanguages list.
	 *
	 * @return the language code from metadata, or empty if not available
	 * @deprecated Use {@link #getDetectedLanguage()} instead. This method only
	 *             retrieves language from metadata and doesn't check
	 *             detectedLanguages.
	 */
	@Deprecated(since = "0.8.0", forRemoval = true)
	public Optional<String> getLanguage() {
		if (this.metadata != null) {
			return Optional.ofNullable((String) this.metadata.get("language"));
		}
		return Optional.empty();
	}

	/**
	 * Get the document creation date from metadata.
	 *
	 * @return the creation date from metadata, or empty if not available
	 * @deprecated Use {@link #getMetadataField(String)} with "created" or
	 *             "modified" instead for more precise date field access.
	 */
	@Deprecated(since = "0.8.0", forRemoval = true)
	public Optional<String> getDate() {
		if (this.metadata != null) {
			return Optional.ofNullable((String) this.metadata.get("date"));
		}
		return Optional.empty();
	}

	/**
	 * Get the document subject from metadata.
	 *
	 * @return the subject from metadata, or empty if not available
	 * @deprecated Use {@link #getMetadataField(String)} with "subject" instead.
	 */
	@Deprecated(since = "0.8.0", forRemoval = true)
	public Optional<String> getSubject() {
		if (this.metadata != null) {
			return Optional.ofNullable((String) this.metadata.get("subject"));
		}
		return Optional.empty();
	}

	/**
	 * Get the total page count from the result.
	 *
	 * <p>
	 * This calls the Rust FFI backend for efficient access to metadata.
	 *
	 * @return the page count, or -1 on error
	 * @since 4.0.0
	 */
	public int getPageCount() {
		if (this.metadata != null) {
			Object pages = this.metadata.get("pages");
			if (pages instanceof Map) {
				Object count = ((Map<?, ?>) pages).get("totalCount");
				if (count instanceof Number) {
					return ((Number) count).intValue();
				}
			}
		}
		return 0;
	}

	/**
	 * Get the total chunk count from the result.
	 *
	 * <p>
	 * Returns the number of text chunks when chunking is enabled.
	 *
	 * @return the chunk count, or 0 if no chunks available
	 * @since 4.0.0
	 */
	public int getChunkCount() {
		if (this.chunks != null) {
			return this.chunks.size();
		}
		return 0;
	}

	/**
	 * Get the detected primary language code.
	 *
	 * <p>
	 * Returns the primary detected language as an ISO 639 code.
	 *
	 * @return the detected language code (e.g., "en", "de"), or empty if not
	 *         detected
	 * @since 4.0.0
	 */
	public Optional<String> getDetectedLanguage() {
		if (this.metadata != null) {
			Object langObj = this.metadata.get("language");
			if (langObj instanceof String lang && !lang.isEmpty()) {
				return Optional.of(lang);
			}
		}

		if (this.detectedLanguages != null && !this.detectedLanguages.isEmpty()) {
			return Optional.of(this.detectedLanguages.get(0));
		}

		return Optional.empty();
	}

	/**
	 * Get a metadata field by name.
	 *
	 * <p>
	 * Supports nested field access with dot notation (e.g., "format.pages").
	 *
	 * @param fieldName
	 *            the field name to retrieve
	 * @return the field value as an Object, or empty if not found
	 * @throws KreuzbergException
	 *             if retrieval fails
	 * @since 4.0.0
	 */
	public Optional<Object> getMetadataField(String fieldName) throws KreuzbergException {
		if (fieldName == null || fieldName.isEmpty()) {
			throw new IllegalArgumentException("fieldName cannot be null or empty");
		}

		if ("title".equals(fieldName)) {
			return Optional.ofNullable(this.metadata.get("title"));
		}
		if ("author".equals(fieldName)) {
			return Optional.ofNullable(this.metadata.get("author"));
		}
		if ("subject".equals(fieldName)) {
			return Optional.ofNullable(this.metadata.get("subject"));
		}
		if ("keywords".equals(fieldName)) {
			return Optional.ofNullable(this.metadata.get("keywords"));
		}
		if ("language".equals(fieldName)) {
			return Optional.ofNullable(this.metadata.get("language"));
		}
		if ("created".equals(fieldName)) {
			return Optional.ofNullable(this.metadata.get("created"));
		}
		if ("modified".equals(fieldName)) {
			return Optional.ofNullable(this.metadata.get("modified"));
		}
		if ("creators".equals(fieldName)) {
			return Optional.ofNullable(this.metadata.get("creators"));
		}
		if ("format".equals(fieldName)) {
			return Optional.ofNullable(this.metadata.get("format"));
		}
		if ("pages".equals(fieldName)) {
			return Optional.ofNullable(this.metadata.get("pages"));
		}

		return Optional.empty();
	}

	@Override
	public String toString() {
		return "ExtractionResult{" + "contentLength=" + content.length() + ", mimeType='" + mimeType + '\''
				+ ", tables=" + tables.size() + ", detectedLanguages=" + detectedLanguages + ", chunks=" + chunks.size()
				+ ", images=" + images.size() + ", pages=" + pages.size() + ", elements=" + elements.size()
				+ ", hasDjotContent=" + (djotContent != null) + '}';
	}
}
