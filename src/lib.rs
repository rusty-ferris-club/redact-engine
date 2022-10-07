//! > **Redact text**
//!
//! ## Usage
#![doc = include_str!("../examples/redaction_string.rs")]
//!
//! ## Example
//!
//! ```console
//! $ cargo run --example redaction_string
//! ```
//!
pub use crate::data::Pattern;
pub use crate::redaction::Redaction;

mod data;
mod engine;
mod redaction;
