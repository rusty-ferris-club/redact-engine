//! > **Redact text**
//!
//! ## Usage
//!
//! ## Example
//!
//! ```console
//! $ cargo run --example redaction_string
//! ```
pub use crate::{data::Pattern, redaction::Redaction};

#[cfg(feature = "redact-json")]
mod json;

mod data;
mod engine;
mod redaction;
