use std::{io, str};

use anyhow::Result;

#[cfg(feature = "redact-json")]
use crate::json;
use crate::{
    data::{Captures, Info, Pattern, REDACT_PLACEHOLDER},
    engine,
};

/// Redact struct
pub struct Redaction {
    patterns: Vec<Pattern>,
    redact_placeholder: String,

    #[cfg(feature = "redact-json")]
    json: json::Redact,
}

impl Default for Redaction {
    /// Create a [`Redaction`] Methods
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text_redaction::Redaction;
    /// Redaction::default()
    /// # ;
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Redaction {
    #[must_use]
    /// Create a [`Redaction`] Methods
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text_redaction::Redaction;
    /// Redaction::new()
    /// # ;
    /// ```
    pub fn new() -> Self {
        Self::custom(REDACT_PLACEHOLDER)
    }

    #[must_use]
    /// Create a [`Redaction`] with redact placeholder text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text_redaction::Redaction;
    /// Redaction::custom("[HIDDEN_VALUE]")
    /// # ;
    /// ```
    pub fn custom(redact_placeholder: &str) -> Self {
        Self {
            patterns: vec![],
            redact_placeholder: redact_placeholder.to_string(),

            #[cfg(feature = "redact-json")]
            json: json::Redact::default(),
        }
    }

    #[must_use]
    /// Add a [`Pattern`] to the redaction list
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text_redaction::{Redaction, Pattern};
    /// # use regex::Regex;
    /// let text = "foo,bar";
    /// let pattern = Pattern {
    ///    test: Regex::new("(bar)").unwrap(),
    ///    group: 1,
    /// };
    ///
    /// Redaction::new().add_pattern(pattern);
    /// # ;
    /// ```
    pub fn add_pattern(mut self, pattern: Pattern) -> Self {
        self.patterns.push(pattern);
        self
    }

    #[cfg(feature = "redact-json")]
    #[must_use]
    pub fn add_key(mut self, key: &str) -> Self {
        self.json = self.json.add_key(key);
        self
    }

    #[cfg(feature = "redact-json")]
    #[must_use]
    pub fn add_path(mut self, key: &str) -> Self {
        self.json = self.json.add_path(key);
        self
    }

    #[must_use]
    /// Add list if [`Pattern`] to the redaction list
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text_redaction::{Redaction, Pattern};
    /// # use regex::Regex;
    /// let text = "foo,bar";
    /// let pattern = Pattern {
    ///    test: Regex::new("(bar)").unwrap(),
    ///    group: 1,
    /// };
    ///
    /// Redaction::new().add_patterns(vec![pattern]);
    /// # ;
    /// ```
    pub fn add_patterns(mut self, patterns: Vec<Pattern>) -> Self {
        self.patterns.extend(patterns);
        self
    }

    #[must_use]
    /// Redact from string
    pub fn redact_str(&self, str: &str) -> String {
        self.redact_patterns(str, false).string
    }

    #[must_use]
    /// Redact from string with extra information of the matches
    pub fn redact_str_with_info(&self, str: &str) -> Info {
        self.redact_patterns(str, true)
    }

    /// Redact text from reader
    ///
    /// # Errors
    /// - When file not exists.
    /// - Could not open reader.
    pub fn redact_reader<R>(&self, rdr: R) -> Result<String>
    where
        R: io::Read,
    {
        let mut rdr_box = Box::new(rdr);
        let mut buffer = Vec::new();
        rdr_box.read_to_end(&mut buffer)?;
        Ok(self.redact_str(str::from_utf8(&buffer)?))
    }

    /// Redact text from reader with extra information of the matches
    ///
    /// # Errors
    /// - When file not exists.
    /// - Could not open reader.
    pub fn redact_reader_with_info<R>(&self, rdr: R) -> Result<Info>
    where
        R: io::Read,
    {
        let mut rdr_box = Box::new(rdr);
        let mut buffer = Vec::new();
        rdr_box.read_to_end(&mut buffer)?;
        Ok(self.redact_str_with_info(str::from_utf8(&buffer)?))
    }

    #[cfg(feature = "redact-json")]
    /// Redact from string
    ///
    /// # Errors
    /// return an error when the given str is not a JSON string
    pub fn redact_json(&self, str: &str) -> Result<String> {
        self.json.redact_str(str)
    }

    /// loop on the pattern list and try to find matches
    fn redact_patterns(&self, str: &str, with_info: bool) -> Info {
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
        let result = engine::try_capture(str, &pattern.test, pattern.group, with_info);
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
}

#[cfg(test)]
mod test_redaction {

    use std::{env, fs::File, io::Write};

    use insta::assert_debug_snapshot;
    use regex::Regex;

    use super::*;

    const TEXT: &str = "foo,bar,baz";

    #[test]
    fn can_redact_str() {
        let pattern = Pattern {
            test: Regex::new("(bar)").unwrap(),
            group: 1,
        };
        let redaction = Redaction::new().add_pattern(pattern);
        assert_debug_snapshot!(redaction.redact_str(TEXT));
    }

    #[test]
    fn can_redact_str_with_info() {
        let pattern = Pattern {
            test: Regex::new("(bar)").unwrap(),
            group: 1,
        };
        let redaction = Redaction::new().add_pattern(pattern);
        assert_debug_snapshot!(redaction.redact_str_with_info(TEXT));
    }

    #[test]
    fn can_redact_reader() {
        let file_path = env::temp_dir().join("foo.txt");

        let mut f = File::create(&file_path).unwrap();
        #[allow(clippy::unused_io_amount)]
        f.write(TEXT.as_bytes()).unwrap();

        let pattern = Pattern {
            test: Regex::new("(bar)").unwrap(),
            group: 1,
        };

        let redaction = Redaction::new().add_pattern(pattern);
        assert_debug_snapshot!(redaction.redact_reader(File::open(file_path).unwrap()));
    }

    #[test]
    fn can_redact_reader_with_info() {
        let file_path = env::temp_dir().join("foo.txt");

        let mut f = File::create(&file_path).unwrap();
        #[allow(clippy::unused_io_amount)]
        f.write(TEXT.as_bytes()).unwrap();

        let pattern = Pattern {
            test: Regex::new("(bar)").unwrap(),
            group: 1,
        };

        let redaction = Redaction::new().add_pattern(pattern);
        assert_debug_snapshot!(redaction.redact_reader_with_info(File::open(file_path).unwrap()));
    }

    #[test]
    fn can_redact_with_multiple_patterns() {
        let bar = Pattern {
            test: Regex::new("(bar)").unwrap(),
            group: 1,
        };
        let baz = Pattern {
            test: Regex::new("(foo),(bar),(baz)").unwrap(),
            group: 3,
        };
        let redaction = Redaction::new().add_patterns(vec![bar, baz]);
        assert_debug_snapshot!(redaction.redact_str(TEXT));
    }

    #[test]
    fn can_redact_with_placeholder_text() {
        let pattern = Pattern {
            test: Regex::new("(bar)").unwrap(),
            group: 1,
        };
        let redaction = Redaction::custom("[HIDDEN_TEXT]").add_pattern(pattern);
        assert_debug_snapshot!(redaction.redact_str(TEXT));
    }
}
