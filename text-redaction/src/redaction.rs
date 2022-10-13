//! redaction function user interface
use std::{io, str};

use anyhow::{bail, Result};
use regex::{escape, Regex};

#[cfg(feature = "redact-info")]
use crate::data::Info;
#[cfg(feature = "redact-json")]
use crate::json;
use crate::{
    data::{Pattern, REDACT_PLACEHOLDER},
    pattern,
};

/// Define redact settings
pub struct Redaction {
    /// Define an option to redact text in JSON schema. enable by `redact-json`
    /// feature flag enabled.
    #[cfg(feature = "redact-json")]
    json: json::Redact,

    /// Define the default redact option by patterns logic.
    pattern: pattern::Redact,
}

impl Default for Redaction {
    /// Create a [`Redaction`] Methods
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_redaction::Redaction;
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
    /// # Example
    ///
    /// ```rust
    /// use text_redaction::Redaction;
    /// Redaction::custom("CUSTOM_HIDDEN_TEXT")
    /// # ;
    /// ```
    pub fn new() -> Self {
        Self::custom(REDACT_PLACEHOLDER)
    }

    #[must_use]
    /// Create a [`Redaction`] with redact placeholder text.
    ///
    /// # Arguments
    /// * `redact_placeholder` - placeholder redaction
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_redaction::Redaction;
    /// Redaction::custom("[HIDDEN_VALUE]")
    /// # ;
    /// ```
    pub fn custom(redact_placeholder: &str) -> Self {
        Self {
            #[cfg(feature = "redact-json")]
            json: json::Redact::with_redact_placeholder(redact_placeholder),

            pattern: pattern::Redact::with_redact_placeholder(redact_placeholder),
        }
    }

    /// redact exact string match
    ///
    /// # Arguments
    /// * `value` - The redaction value
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_redaction::Redaction;
    /// let text = "foo,bar";
    /// Redaction::new().add_value("foo");
    /// # ;
    /// ```
    /// # Errors
    /// when the value could not converted to a regex
    pub fn add_value(self, value: &str) -> Result<Self> {
        let pattern = Pattern {
            test: Regex::new(&format!("({})", escape(value)))?,
            group: 1,
        };

        Ok(self.add_pattern(pattern))
    }

    /// redact exact string match from list of strings
    ///
    /// # Arguments
    /// * `values` - List of redaction value
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_redaction::Redaction;
    /// let text = "foo,bar,baz";
    /// Redaction::new().add_values(vec!["foo", "baz"]);
    /// # ;
    /// ```
    /// # Errors
    /// when the value could not converted to a regex
    pub fn add_values(self, values: Vec<&str>) -> Result<Self> {
        let mut errors = vec![];

        let patterns = values
            .iter()
            .filter_map(|val| match Regex::new(&format!("({})", escape(val))) {
                Ok(test) => Some(Pattern { test, group: 1 }),
                Err(_e) => {
                    errors.push((*val).to_string());
                    None
                }
            })
            .collect::<Vec<_>>();

        if !errors.is_empty() {
            bail!("could not parse {} to regex", errors.join(","))
        }

        Ok(self.add_patterns(patterns))
    }

    #[must_use]
    /// Add a [`Pattern`] to the redaction list
    ///
    /// # Arguments
    /// * `pattern` - redact [Pattern]
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_redaction::{Redaction, Pattern};
    /// use regex::Regex;
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
        self.pattern = self.pattern.add_pattern(pattern);
        self
    }

    #[must_use]
    /// Add list if [`Pattern`] to the redaction list
    ///
    /// # Arguments
    /// * `patterns` - List of redact [Pattern]
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_redaction::{Redaction, Pattern};
    /// use regex::Regex;
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
        self.pattern = self.pattern.add_patterns(patterns);
        self
    }

    #[cfg(feature = "redact-json")]
    #[must_use]
    /// Redact the JSON value of the given key. enable by `redact-json`
    ///
    /// # Optional
    /// When `redact-json` feature flag is enabled
    ///
    /// # Arguments
    /// * `key` -  The JSON key
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_redaction::Redaction;
    /// Redaction::new().add_key("bar").add_key("array");
    /// # ;
    /// ```
    pub fn add_key(mut self, key: &str) -> Self {
        self.json = self.json.add_key(key);
        self
    }

    #[cfg(feature = "redact-json")]
    #[must_use]
    /// Redact the JSON by JSON path. enable by `redact-json`.
    ///
    /// # Optional
    /// When `redact-json` feature flag is enabled
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_redaction::Redaction;
    /// Redaction::new().add_key("bar").add_key("array");
    /// # ;
    /// ```
    pub fn add_path(mut self, key: &str) -> Self {
        self.json = self.json.add_path(key);
        self
    }

    #[must_use]
    /// Redact from string
    pub fn redact_str(&self, str: &str) -> String {
        self.pattern.redact_patterns(str, false).string
    }

    #[cfg(feature = "redact-info")]
    #[must_use]
    /// Redact from string with extra information of the matches
    ///
    /// # Optional
    /// When `redact-info` feature flag is enabled
    pub fn redact_str_with_info(&self, str: &str) -> Info {
        self.pattern.redact_patterns(str, true)
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
    /// # Optional
    /// When `redact-info` feature flag is enabled
    ///
    /// # Errors
    /// - When file not exists.
    /// - Could not open reader.
    #[cfg(feature = "redact-info")]
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
    /// Redact from string.
    ///
    /// # Optional
    /// When `redact-json` feature flag is enabled
    ///
    /// # Errors
    /// return an error when the given str is not a JSON string
    pub fn redact_json(&self, str: &str) -> Result<String> {
        self.json.redact_str(&self.redact_str(str))
    }
}

#[cfg(test)]
mod test_redaction {

    use std::{env, fs::File, io::Write};

    use insta::assert_debug_snapshot;

    use super::*;

    const TEXT: &str = "foo,bar,baz,extra";

    #[cfg(feature = "redact-json")]
    use serde_json::json;

    #[test]
    fn test_by_pattern() {
        let pattern = Pattern {
            test: Regex::new("(foo)").unwrap(),
            group: 1,
        };
        let patterns = vec![
            Pattern {
                test: Regex::new("(bar)").unwrap(),
                group: 1,
            },
            Pattern {
                test: Regex::new("(baz)").unwrap(),
                group: 1,
            },
        ];
        assert_debug_snapshot!(Redaction::new()
            .add_pattern(pattern)
            .add_patterns(patterns)
            .redact_str(TEXT));
    }

    #[test]
    fn test_bt_value() {
        assert_debug_snapshot!(Redaction::new()
            .add_value("foo")
            .unwrap()
            .add_values(vec!["bar", "baz"])
            .unwrap()
            .redact_str(TEXT));
    }

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
    #[cfg(feature = "redact-info")]
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
    #[cfg(feature = "redact-info")]
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
        let patterns = vec![
            Pattern {
                test: Regex::new("(bar)").unwrap(),
                group: 1,
            },
            Pattern {
                test: Regex::new("(foo),(bar),(baz)").unwrap(),
                group: 3,
            },
        ];

        let redaction = Redaction::new().add_patterns(patterns);
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

    #[test]
    #[cfg(feature = "redact-json")]
    fn can_redact_json() {
        let pattern = Pattern {
            test: Regex::new("(redact-by-pattern)").unwrap(),
            group: 1,
        };

        let json = json!({
        "all-path": {
            "b": {
                "key": "redact_me",
            },
            "foo": "redact_me",
            "key": "redact_me",
        },
        "specific-key": {
            "b": {
                "key": "skip-redaction",
            },
            "foo": "skip-redaction",
            "key": "redact_me"
        },
        "key": "redact_me",
        "skip": "skip-redaction",
        "by-value": "bar",
        "by-pattern": "redact-by-pattern",
        })
        .to_string();

        let redaction = Redaction::default()
            .add_pattern(pattern)
            .add_path("all-path.*")
            .add_path("specific-key.key")
            .add_key("key")
            .add_value("bar")
            .unwrap();
        assert_debug_snapshot!(redaction.redact_json(&json));
    }
}
