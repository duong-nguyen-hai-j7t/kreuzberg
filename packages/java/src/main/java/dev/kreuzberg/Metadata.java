package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

/**
 * Metadata extracted from a document.
 *
 * <p>
 * Contains common fields applicable to all formats (title, subject, authors,
 * keywords, language, timestamps, creators) extracted from document properties.
 *
 * @since 0.8.0
 */
public final class Metadata {
	private final Optional<String> title;
	private final Optional<String> subject;
	private final Optional<List<String>> authors;
	private final Optional<List<String>> keywords;
	private final Optional<String> language;
	private final Optional<String> createdAt;
	private final Optional<String> modifiedAt;
	private final Optional<String> creator;
	private final Optional<Map<String, Object>> additional;

	@JsonCreator
	public Metadata(@JsonProperty("title") Optional<String> title, @JsonProperty("subject") Optional<String> subject,
			@JsonProperty("authors") Optional<List<String>> authors,
			@JsonProperty("keywords") Optional<List<String>> keywords,
			@JsonProperty("language") Optional<String> language, @JsonProperty("created_at") Optional<String> createdAt,
			@JsonProperty("modified_at") Optional<String> modifiedAt, @JsonProperty("creator") Optional<String> creator,
			@JsonProperty("additional") Optional<Map<String, Object>> additional) {
		this.title = title != null ? title : Optional.empty();
		this.subject = subject != null ? subject : Optional.empty();
		this.authors = authors != null && authors.isPresent()
				? Optional.of(Collections.unmodifiableList(new ArrayList<>(authors.get())))
				: Optional.empty();
		this.keywords = keywords != null && keywords.isPresent()
				? Optional.of(Collections.unmodifiableList(new ArrayList<>(keywords.get())))
				: Optional.empty();
		this.language = language != null ? language : Optional.empty();
		this.createdAt = createdAt != null ? createdAt : Optional.empty();
		this.modifiedAt = modifiedAt != null ? modifiedAt : Optional.empty();
		this.creator = creator != null ? creator : Optional.empty();
		this.additional = additional != null ? additional : Optional.empty();
	}

	/**
	 * Creates a new empty Metadata.
	 *
	 * @return a new empty Metadata instance
	 */
	public static Metadata empty() {
		return new Metadata(Optional.empty(), Optional.empty(), Optional.empty(), Optional.empty(), Optional.empty(),
				Optional.empty(), Optional.empty(), Optional.empty(), Optional.empty());
	}

	/**
	 * Get the document title.
	 *
	 * @return optional title
	 */
	public Optional<String> getTitle() {
		return title;
	}

	/**
	 * Get the document subject or description.
	 *
	 * @return optional subject
	 */
	public Optional<String> getSubject() {
		return subject;
	}

	/**
	 * Get the document authors.
	 *
	 * @return optional unmodifiable list of authors
	 */
	public Optional<List<String>> getAuthors() {
		return authors;
	}

	/**
	 * Get the document keywords/tags.
	 *
	 * @return optional unmodifiable list of keywords
	 */
	public Optional<List<String>> getKeywords() {
		return keywords;
	}

	/**
	 * Get the primary language code (ISO 639).
	 *
	 * @return optional language code (e.g., "en", "de")
	 */
	public Optional<String> getLanguage() {
		return language;
	}

	/**
	 * Get the creation timestamp (ISO 8601 format).
	 *
	 * @return optional creation timestamp
	 */
	public Optional<String> getCreatedAt() {
		return createdAt;
	}

	/**
	 * Get the last modification timestamp (ISO 8601 format).
	 *
	 * @return optional modification timestamp
	 */
	public Optional<String> getModifiedAt() {
		return modifiedAt;
	}

	/**
	 * Get the document creator.
	 *
	 * @return optional creator name
	 */
	public Optional<String> getCreator() {
		return creator;
	}

	/**
	 * Get additional format-specific metadata.
	 *
	 * @return optional metadata map with additional fields
	 */
	public Optional<Map<String, Object>> getAdditional() {
		return additional;
	}

	/**
	 * Check if any metadata is present.
	 *
	 * @return true if at least one field is present
	 */
	public boolean isEmpty() {
		return !title.isPresent() && !subject.isPresent() && !authors.isPresent() && !keywords.isPresent()
				&& !language.isPresent() && !createdAt.isPresent() && !modifiedAt.isPresent() && !creator.isPresent()
				&& !additional.isPresent();
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof Metadata)) {
			return false;
		}
		Metadata other = (Metadata) obj;
		return Objects.equals(title, other.title) && Objects.equals(subject, other.subject)
				&& Objects.equals(authors, other.authors) && Objects.equals(keywords, other.keywords)
				&& Objects.equals(language, other.language) && Objects.equals(createdAt, other.createdAt)
				&& Objects.equals(modifiedAt, other.modifiedAt) && Objects.equals(creator, other.creator)
				&& Objects.equals(additional, other.additional);
	}

	@Override
	public int hashCode() {
		return Objects.hash(title, subject, authors, keywords, language, createdAt, modifiedAt, creator, additional);
	}

	@Override
	public String toString() {
		return "Metadata{" + "title=" + title + ", subject=" + subject + ", authors=" + authors + ", keywords="
				+ keywords + ", language=" + language + ", createdAt=" + createdAt + ", modifiedAt=" + modifiedAt
				+ ", creator=" + creator + ", additional=" + additional + '}';
	}
}
