package dev.kreuzberg.config;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * PDF-specific extraction options.
 *
 * @since 4.0.0
 */
public final class PdfConfig {
	private final boolean extractImages;
	private final List<String> passwords;
	private final boolean extractMetadata;
	private final FontConfig fontConfig;
	private final HierarchyConfig hierarchyConfig;
	private final boolean extractAnnotations;
	private final Double topMarginFraction;
	private final Double bottomMarginFraction;

	private PdfConfig(Builder builder) {
		this.extractImages = builder.extractImages;
		this.passwords = builder.passwords != null
				? Collections.unmodifiableList(new ArrayList<>(builder.passwords))
				: null;
		this.extractMetadata = builder.extractMetadata;
		this.fontConfig = builder.fontConfig;
		this.hierarchyConfig = builder.hierarchyConfig;
		this.extractAnnotations = builder.extractAnnotations;
		this.topMarginFraction = builder.topMarginFraction;
		this.bottomMarginFraction = builder.bottomMarginFraction;
	}

	public static Builder builder() {
		return new Builder();
	}

	public boolean isExtractImages() {
		return extractImages;
	}

	public List<String> getPasswords() {
		return passwords;
	}

	public boolean isExtractMetadata() {
		return extractMetadata;
	}

	public FontConfig getFontConfig() {
		return fontConfig;
	}

	public HierarchyConfig getHierarchyConfig() {
		return hierarchyConfig;
	}

	public boolean isExtractAnnotations() {
		return extractAnnotations;
	}

	public Double getTopMarginFraction() {
		return topMarginFraction;
	}

	public Double getBottomMarginFraction() {
		return bottomMarginFraction;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		map.put("extract_images", extractImages);
		if (passwords != null && !passwords.isEmpty()) {
			map.put("passwords", passwords);
		}
		map.put("extract_metadata", extractMetadata);
		map.put("extract_annotations", extractAnnotations);
		if (fontConfig != null) {
			map.put("font_config", fontConfig.toMap());
		}
		if (hierarchyConfig != null) {
			map.put("hierarchy", hierarchyConfig.toMap());
		}
		if (topMarginFraction != null) {
			map.put("top_margin_fraction", topMarginFraction);
		}
		if (bottomMarginFraction != null) {
			map.put("bottom_margin_fraction", bottomMarginFraction);
		}
		return map;
	}

	public static final class Builder {
		private boolean extractImages = false;
		private List<String> passwords;
		private boolean extractMetadata = true;
		private FontConfig fontConfig;
		private HierarchyConfig hierarchyConfig;
		private boolean extractAnnotations = false;
		private Double topMarginFraction;
		private Double bottomMarginFraction;

		private Builder() {
		}

		public Builder extractImages(boolean extractImages) {
			this.extractImages = extractImages;
			return this;
		}

		public Builder passwords(List<String> passwords) {
			this.passwords = passwords;
			return this;
		}

		public Builder password(String password) {
			if (this.passwords == null) {
				this.passwords = new ArrayList<>();
			}
			this.passwords.add(password);
			return this;
		}

		public Builder extractMetadata(boolean extractMetadata) {
			this.extractMetadata = extractMetadata;
			return this;
		}

		public Builder fontConfig(FontConfig fontConfig) {
			this.fontConfig = fontConfig;
			return this;
		}

		public Builder hierarchyConfig(HierarchyConfig hierarchyConfig) {
			this.hierarchyConfig = hierarchyConfig;
			return this;
		}

		public Builder extractAnnotations(boolean extractAnnotations) {
			this.extractAnnotations = extractAnnotations;
			return this;
		}

		public Builder topMarginFraction(Double topMarginFraction) {
			this.topMarginFraction = topMarginFraction;
			return this;
		}

		public Builder bottomMarginFraction(Double bottomMarginFraction) {
			this.bottomMarginFraction = bottomMarginFraction;
			return this;
		}

		public PdfConfig build() {
			return new PdfConfig(this);
		}
	}

	static PdfConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		Object extractImagesValue = map.get("extract_images");
		if (extractImagesValue instanceof Boolean) {
			builder.extractImages((Boolean) extractImagesValue);
		}
		Object passwordsValue = map.get("passwords");
		if (passwordsValue instanceof Iterable) {
			@SuppressWarnings("unchecked")
			Iterable<Object> iterable = (Iterable<Object>) passwordsValue;
			List<String> passwords = new ArrayList<>();
			for (Object entry : iterable) {
				if (entry instanceof String) {
					passwords.add((String) entry);
				}
			}
			builder.passwords(passwords);
		}
		Object extractMetadataValue = map.get("extract_metadata");
		if (extractMetadataValue instanceof Boolean) {
			builder.extractMetadata((Boolean) extractMetadataValue);
		}
		Object extractAnnotationsValue = map.get("extract_annotations");
		if (extractAnnotationsValue instanceof Boolean) {
			builder.extractAnnotations((Boolean) extractAnnotationsValue);
		}
		Object topMarginValue = map.get("top_margin_fraction");
		if (topMarginValue instanceof Number) {
			builder.topMarginFraction(((Number) topMarginValue).doubleValue());
		}
		Object bottomMarginValue = map.get("bottom_margin_fraction");
		if (bottomMarginValue instanceof Number) {
			builder.bottomMarginFraction(((Number) bottomMarginValue).doubleValue());
		}
		@SuppressWarnings("unchecked")
		Map<String, Object> fontConfigMap = map.get("font_config") instanceof Map
				? (Map<String, Object>) map.get("font_config")
				: null;
		if (fontConfigMap != null) {
			builder.fontConfig(FontConfig.fromMap(fontConfigMap));
		}
		@SuppressWarnings("unchecked")
		Map<String, Object> hierarchyConfigMap = map.get("hierarchy") instanceof Map
				? (Map<String, Object>) map.get("hierarchy")
				: null;
		if (hierarchyConfigMap != null) {
			builder.hierarchyConfig(HierarchyConfig.fromMap(hierarchyConfigMap));
		}
		return builder.build();
	}
}
