//! Build system/user prompts from a resolved preset.
//!
//! TODO(Wave 2, agent J): implement `BuiltPrompt`, `build(...)` and `build_vision_fallback(...)`.
//! Generic `{{var}}` substitution over [`super::StructuredOptions::context`] (leave unknown vars in
//! place); ship a GENERIC default citation instruction (tuned wording stays in cloud presets). Take
//! `max_excerpt_chars` from [`super::VisionConfig`] — no env reads. Port the cloud tests.
