use serde_derive::Deserialize;

/// Default redact placeholder
pub const REDACT_PLACEHOLDER: &str = "[TEXT_REDACTED]";

#[derive(Debug, Deserialize, Clone)]
pub struct Pattern {
    #[serde(with = "serde_regex")]
    pub test: regex::Regex,
    pub group: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Info {
    pub string: String,
    pub captures: Vec<Captures>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Captures {
    pub text: String,
    pub position: Option<Position>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Position {
    pub line_number: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}
