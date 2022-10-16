//! Common structs
use serde_derive::Deserialize;

/// Default redact placeholder
pub const REDACT_PLACEHOLDER: &str = "[TEXT_REDACTED]";

#[derive(Debug, Deserialize, Clone)]
/// Describe redaction by Pattern
pub struct Pattern {
    #[serde(with = "serde_regex")]
    /// regex Pattern
    pub test: regex::Regex,
    /// capture group to redact
    pub group: usize,
}

#[derive(Debug, Deserialize, Clone)]
/// Redact information
pub struct Info {
    /// redacted string
    pub string: String,
    /// captures information
    pub captures: Vec<Captures>,
}

#[derive(Debug, Deserialize, Clone)]
/// Capture details
pub struct Captures {
    /// the captured text
    pub text: String,
    /// Position capture details
    pub position: Option<Position>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Position {
    /// capture line number
    pub line: usize,
    /// start caption position
    pub start_offset: usize,
    /// end caption position
    pub end_offset: usize,
}
