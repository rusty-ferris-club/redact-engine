use regex::Regex;

use crate::data::{Captures, Info, Pattern, Position, REDACT_PLACEHOLDER};

pub struct Redact {
    /// redact placeholder text
    pub redact_placeholder: String,

    patterns: Vec<Pattern>,
}

impl Default for Redact {
    /// Create a [`Redact`] Methods
    fn default() -> Self {
        Self::new(REDACT_PLACEHOLDER, vec![])
    }
}

impl Redact {
    pub fn with_redact_template(redact_placeholder: &str) -> Self {
        Self::new(redact_placeholder, vec![])
    }
    /// Create a [`Redact`] Methods with all available fields
    pub fn new(redact_placeholder: &str, patterns: Vec<Pattern>) -> Self {
        Self {
            redact_placeholder: redact_placeholder.to_string(),
            patterns,
        }
    }

    pub fn add_pattern(mut self, pattern: Pattern) -> Self {
        self.patterns.push(pattern);
        self
    }

    pub fn add_patterns(mut self, patterns: Vec<Pattern>) -> Self {
        self.patterns.extend(patterns);
        self
    }

    /// loop on the pattern list and try to find matches
    pub fn redact_patterns(&self, str: &str, with_info: bool) -> Info {
        let mut text_results = str.to_owned();

        let captures = self
            .patterns
            .iter()
            .filter_map(|pattern| {
                let result = Self::redact_by_pattern(str, pattern, with_info);
                result.map(|findings| {
                    for c in &findings {
                        text_results = text_results.replacen(&c.text, &self.redact_placeholder, 1);
                    }
                    findings
                })
            })
            .flatten()
            .collect::<Vec<_>>();

        Info {
            string: text_results,
            captures,
        }
    }

    fn redact_by_pattern(str: &str, pattern: &Pattern, with_info: bool) -> Option<Vec<Captures>> {
        let result = Self::try_capture(str, &pattern.test, pattern.group, with_info);
        if result.is_empty() {
            None
        } else {
            Some(
                result
                    .iter()
                    .map(|(finding_text, position)| Captures {
                        text: finding_text.to_string(),
                        position: position.clone(),
                    })
                    .collect::<Vec<_>>(),
            )
        }
    }

    /// Check if the given regex match to the given string
    fn try_capture(
        str: &str,
        re: &Regex,
        group: usize,
        with_info: bool,
    ) -> Vec<(String, Option<Position>)> {
        re.captures_iter(str)
            .filter_map(|cap| {
                cap.get(group).map(|m| {
                    let more_info = if with_info {
                        Some(Position {
                            line_number: 1,
                            start_offset: m.start(),
                            end_offset: m.end(),
                        })
                    } else {
                        None
                    };

                    (m.as_str().to_string(), more_info)
                })
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test_pattern {

    use std::env;

    use insta::assert_debug_snapshot;

    use super::*;

    const TEXT: &str = "foo,bar,baz,foo";

    #[test]
    fn can_redact_patterns() {
        let pattern = Pattern {
            test: Regex::new("(bar)").unwrap(),
            group: 1,
        };
        let redaction = Redact::default().add_pattern(pattern);
        assert_debug_snapshot!(redaction.redact_patterns(TEXT, false));
    }

    #[test]
    fn can_redact_multiple_patterns() {
        let pattern1 = Pattern {
            test: Regex::new("(bar)").unwrap(),
            group: 1,
        };
        let pattern2 = Pattern {
            test: Regex::new("(baz),(foo)").unwrap(),
            group: 2,
        };
        let redaction = Redact::default().add_patterns(vec![pattern1, pattern2]);
        assert_debug_snapshot!(redaction.redact_patterns(TEXT, false));
    }

    #[test]
    fn can_redact_patterns_with_info() {
        let pattern = Pattern {
            test: Regex::new("(bar)").unwrap(),
            group: 1,
        };
        let redaction = Redact::default().add_pattern(pattern);
        assert_debug_snapshot!(redaction.redact_patterns(TEXT, true));
    }

    #[test]
    fn can_try_capture() {
        assert_debug_snapshot!(Redact::try_capture(
            TEXT,
            &Regex::new("(foo)").unwrap(),
            1,
            false
        ));
    }

    #[test]
    fn can_try_capture_with_info() {
        assert_debug_snapshot!(Redact::try_capture(
            TEXT,
            &Regex::new("(foo)").unwrap(),
            1,
            true
        ));
    }
}
