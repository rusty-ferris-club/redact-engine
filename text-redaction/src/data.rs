use serde_derive::Deserialize;

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
    pub position: Position,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Position {
    pub line_number: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}
