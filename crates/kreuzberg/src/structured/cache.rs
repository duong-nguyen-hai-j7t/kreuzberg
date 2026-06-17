//! Vision-call caching.
//!
//! The [`VisionCallCache`] trait lets callers deduplicate identical vision LLM calls across runs.
//! The in-process [`MokaVisionCache`] is the default; the cloud injects a distributed (NATS-backed)
//! implementation. The trait object is intentionally excluded from the FFI/binding surface — bindings
//! get the cache-less path.

/// Identifies a single vision call for caching.
///
/// Two calls with equal keys are guaranteed to produce equivalent vision responses, so a cache hit
/// can be returned without re-calling the model.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    /// Hash of the rendered page bytes covered by this call.
    pub content_hash: String,
    /// Inclusive 1-indexed page range `(first, last)` covered by this call.
    pub page_range: (u32, u32),
    /// Fingerprint of the resolved preset (schema + prompt + settings).
    pub preset_fingerprint: String,
    /// Hash of the fully-built prompt.
    pub prompt_hash: String,
    /// Model identifier used for the call.
    pub model: String,
}

/// A cache for vision LLM call results, keyed by [`CacheKey`].
///
/// Implementations must be cheap to call and safe to share across threads. `get` returns the cached
/// structured JSON value when present; `put` stores a freshly computed value.
pub trait VisionCallCache: Send + Sync + std::fmt::Debug {
    /// Look up a cached vision response.
    fn get(&self, key: &CacheKey) -> Option<serde_json::Value>;
    /// Store a vision response.
    fn put(&self, key: CacheKey, value: serde_json::Value);
}

// TODO(Wave 1, agent E): add `MokaVisionCache` (in-process `moka::sync::Cache`-backed impl) and its
// unit tests here. The `structured` feature already pulls in `dep:moka`.
