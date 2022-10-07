//! > **Redact text**
//!
//! ## Usage
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
