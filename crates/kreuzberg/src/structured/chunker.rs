//! Token-aware batch packing of rendered pages into vision calls.
//!
//! TODO(Wave 1, agent B): implement `Batch`, `ChunkerConfig` (with `Default`, NO env reads) and
//! `batch_pages(pages, user_text, &ChunkerConfig) -> Vec<Batch>`. Greedy token-aware packing with
//! the cloud constants as config defaults: `CHARS_PER_TOKEN = 4`, `IMAGE_TOKEN_ESTIMATE = 1500`,
//! `max_input_tokens = 800_000`. `user_text` rides only the first batch. Port the cloud tests.
