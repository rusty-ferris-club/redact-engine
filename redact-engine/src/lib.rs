//! **Redact text**
//! Protect confidentiality with dynamic redaction by replacing sensitive data
//! from string or JSON format
//!
//! ## Usecase
//! - Redact sensitive information from a text file
//! - Redact JSON schema by providing
//!    - By key
//!    - Prefix path
//! - Integrate with [env_logger](https://github.com/rusty-ferris-club/redact-engine/tree/main/redact-engine/examples/logger/env-logger)
//!
//! ## Usage
//! ```
#![doc = include_str!("../examples/redaction_values.rs")]
//!
//! ```
//! To see all code [example](https://github.com/rusty-ferris-club/redact-engine/tree/main/redact-engine/examples)
pub use crate::{data::Pattern, redaction::Redaction};

#[cfg(feature = "redact-json")]
mod json;

mod data;
mod pattern;
mod redaction;
