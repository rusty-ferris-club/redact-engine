//! Patterns redaction by [Pattern]
//!
//! # Example:
//! ```
#![doc = include_str!("../examples/redaction_string.rs")]
//! ```
//!
use rayon::prelude::*;
use regex::Regex;

use crate::data::{Captures, Info, Pattern, Position, REDACT_PLACEHOLDER};

/// Define pattern
pub struct Redact {
    /// redact placeholder text
    pub text_placeholder: String,
    /// list of [Pattern]
    patterns: Vec<Pattern>,
}

impl Default for Redact {
    /// Create a [`Redact`] Methods
    fn default() -> Self {
        Self::new(REDACT_PLACEHOLDER, vec![])
    }
}

impl Redact {
    /// Create a [`Redact`] Methods with custom redact placeholder
    ///
    /// # Arguments
    /// * `text_placeholder` - placeholder redaction
    pub fn with_redact_placeholder(text_placeholder: &str) -> Self {
        Self::new(text_placeholder, vec![])
    }

    /// Create a [`Redact`] Methods with all available fields
    ///
    /// # Arguments
    /// * `text_placeholder` - placeholder redaction
    /// * `patterns` - Vec of [Pattern]
    pub fn new(text_placeholder: &str, patterns: Vec<Pattern>) -> Self {
        Self {
            text_placeholder: text_placeholder.to_string(),
            patterns,
        }
    }

    /// Add [Pattern]
    ///
    /// # Arguments
    /// * `pattern` - single [Pattern]
    pub fn add_pattern(mut self, pattern: Pattern) -> Self {
        self.patterns.push(pattern);
        self
    }

    /// Add list of [Pattern]
    ///
    /// # Arguments
    /// * `patterns` - Vec of [Pattern]
    pub fn add_patterns(mut self, patterns: Vec<Pattern>) -> Self {
        self.patterns.extend(patterns);
        self
    }

    /// loop on the [Pattern] vector and try to find matches
    ///
    /// # Arguments
    /// * `str` - is the redact login going to search on
    /// * `with_info` - Adding extra match details to the response. supported
    ///   only when `redact-info` feature flag is enabled
    pub fn redact_patterns(&self, str: &str, with_info: bool) -> Info {
        let mut text_results = str.to_owned();

        let captures = self
            .patterns
            .par_iter()
            .filter_map(|pattern| Self::redact_by_pattern(str, pattern, with_info))
            .flatten()
            .collect::<Vec<_>>();

        for c in &captures {
            text_results = text_results.replacen(&c.text, &self.text_placeholder, 1);
        }
        Info {
            string: text_results,
            captures,
        }
    }

    /// Redact from a single [Pattern]
    ///
    /// # Arguments
    /// * `str` - is the redact login going to search on
    /// * `pattern` - [Pattern] settings
    /// * `with_info` - Adding extra match details to the response. supported
    ///   only when `redact-info` feature flag is enabled
    fn redact_by_pattern(str: &str, pattern: &Pattern, with_info: bool) -> Option<Vec<Captures>> {
        let result = Self::try_capture(str, &pattern.test, pattern.group, with_info);
        if result.is_empty() {
            None
        } else {
            Some(
                result
                    .iter()
                    .filter(|(finding_text, _)| !finding_text.is_empty())
                    .map(|(finding_text, position)| Captures {
                        text: finding_text.to_string(),
                        test: format!("{}", pattern.test),
                        position: position.clone(),
                    })
                    .collect::<Vec<_>>(),
            )
        }
    }

    /// Try to capture matches by the given regex
    ///
    /// # Arguments
    /// * `str` - is the redact login going to search on
    /// * `re` - [regex::Regex] rule
    /// * `with_info` - Adding extra match details to the response. supported
    ///   only when `redact-info` feature flag is enabled
    fn try_capture(
        str: &str,
        re: &Regex,
        group: usize,
        #[allow(unused_variables)] with_info: bool,
    ) -> Vec<(String, Option<Position>)> {
        re.captures_iter(str)
            .filter_map(|cap| {
                cap.get(group).map(|m| {
                    #[cfg(not(feature = "redact-info"))]
                    let more_info = None;
                    #[cfg(feature = "redact-info")]
                    let more_info = if with_info {
                        Some(Position {
                            line: bytecount::count(&str.as_bytes()[..m.start()], 0x0A) + 1,
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
    fn can_redact_patterns_empty_string() {
        let pattern = Pattern {
            test: Regex::new("(?i)CARGO_.*=(.*)").unwrap(),
            group: 1,
        };
        let redaction = Redact::default().add_pattern(pattern);
        assert_debug_snapshot!(redaction.redact_patterns("CARGO_PKG_DESCRIPTION=", false));
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
    #[cfg(feature = "redact-info")]
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

    #[cfg(feature = "redact-info")]
    #[test]
    fn can_try_capture_with_info() {
        let text = r#"bar
        foo
        baz
        foo
        "#;
        assert_debug_snapshot!(Redact::try_capture(
            text,
            &Regex::new("(foo)").unwrap(),
            1,
            true
        ));
    }
}
