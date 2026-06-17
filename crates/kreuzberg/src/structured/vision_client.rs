//! Vision LLM request/response adapter over `liter-llm`.
//!
//! TODO(Wave 2, agent K): implement `VisionRequest`, `VisionResponse` (carrying a `serde_json::Value`
//! plus [`crate::types::LlmUsage`]) and an internal `call(&DefaultClient, VisionRequest)` built from
//! the `liter-llm` API: `Message::System`/`Message::User` with `UserContent::Parts`,
//! `ContentPart::ImageUrl { ImageUrl { url: data-url, detail: Some(ImageDetail::Auto) } }`, and
//! `ResponseFormat::JsonSchema { JsonSchemaFormat { strict: Some(true), .. } }`. Obtain the client
//! via [`crate::llm::client::create_client`] (keep `liter_llm` types off the public surface). Unit
//! test with `wiremock`, pointing [`crate::core::config::LlmConfig::base_url`] at the mock server.
