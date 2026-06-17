//! JSON Schema validation and multi-batch merge of vision responses.
//!
//! TODO(Wave 1, agent C): implement `validate_and_merge(Vec<Value>, &schema, MergeMode)
//! -> MergedOutput` and the `Outcome` enum (`Success`/`PartialSuccess`/`SchemaInvalid`/`Error`).
//! Use the `jsonschema` 0.46 API (`jsonschema::validator_for`, `validator.iter_errors`); stringify
//! the `#[non_exhaustive]` `ValidationError` (never match its internals). `MergeMode` is
//! [`crate::core::config::MergeMode`] — do not introduce a second merge type. Port the cloud tests.
