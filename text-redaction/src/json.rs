//! Redact text from JSON format
//!
//! # Optional
//! This requires `serde_json` feature to be enabled.
use anyhow::Result;
use serde_json::Value;

use crate::data::REDACT_PLACEHOLDER;

pub struct Redact {
    /// redact placeholder text
    pub text_placeholder: String,
    /// list of keys to reduct
    pub keys: Vec<String>,
    /// list of JSON specific path
    pub path: Vec<String>,
    /// list of JSON prefix path
    pub path_prefix: Vec<String>,
}

impl Default for Redact {
    /// Create a [`Redact`] Methods
    fn default() -> Self {
        Self::new(REDACT_PLACEHOLDER, vec![], vec![], vec![])
    }
}

impl Redact {
    pub fn with_redact_placeholder(text_placeholder: &str) -> Self {
        Self::new(text_placeholder, vec![], vec![], vec![])
    }

    /// Create a [`Redact`] Methods with all available fields
    pub fn new(
        text_placeholder: &str,
        keys: Vec<String>,
        path: Vec<String>,
        path_prefix: Vec<String>,
    ) -> Self {
        Self {
            text_placeholder: text_placeholder.to_string(),
            keys,
            path,
            path_prefix,
        }
    }

    /// redact JSON value bt givenkeys
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text_redaction::Redaction;
    /// let text = "foo,bar";
    ///
    /// Redaction::new().add_keys(vec!["key"]);
    /// # ;
    /// ```
    pub fn add_keys(mut self, keys: Vec<&str>) -> Self {
        self.keys.extend(keys.iter().map(|&s| s.to_string()));
        self
    }

    /// redact JSON value by specific key path list
    ///
    /// # Example:
    /// ## Redact by specific key
    /// {
    ///     "a": {
    ///         "b": {
    ///             "key": "redact_me",
    ///         },
    ///         "foo": "bar",
    ///         "key": "skip-redaction"
    ///     },
    ///     "key": "skip-redaction"
    /// }
    /// ```rust
    /// # use text_redaction::{Redaction, Pattern};
    /// # use regex::Regex;
    ///
    /// Redaction::new().add_paths(vec!["a.b.key"]);
    /// # ;
    /// ```
    /// ## Redact all keys under `a`
    /// {
    ///     "a": {
    ///         "b": {
    ///             "key": "redact_me",
    ///         },
    ///         "foo": "bar",
    ///         "key": "skip-redaction"
    ///     },
    ///     "key": "skip-redaction"
    /// }
    /// ```rust
    /// # use text_redaction::{Redaction, Pattern};
    /// # use regex::Regex;
    ///
    /// Redaction::new().add_paths(vec!["a.*"]);
    /// # ;
    pub fn add_paths(mut self, path: Vec<&str>) -> Self {
        for path in path.iter() {
            if path.ends_with('*') {
                self.path_prefix.push((*path).to_string().replace(".*", ""));
            } else {
                self.path.push((*path).to_string());
            }
        }

        self
    }

    /// redact json str
    pub fn redact_str(&self, str: &str) -> Result<String> {
        let mut json_value: Value = serde_json::from_str(str)?;
        // let start_path = String::new();
        self.redact_value(&mut json_value, String::new());
        Ok(json_value.to_string())
    }

    /// redact Value values
    fn redact_value(&self, json: &mut Value, path: String) {
        if let Some(obj) = json.as_object_mut() {
            obj.iter_mut().for_each(|(key, value)| {
                let mut obj_path = path.clone();
                // create a json key path to be able redact by path keys.
                if obj_path.is_empty() {
                    obj_path.push_str(&key.to_string());
                } else {
                    obj_path.push_str(&format!(".{}", key));
                };

                if self.path.contains(&obj_path) || self.path_prefix.contains(&obj_path) {
                    *value = Value::String(self.text_placeholder.to_string());
                } else if self.keys.contains(key) {
                    if value.is_array() {
                        self.redact_value_array(value);
                    } else {
                        *value = Value::String(self.text_placeholder.to_string());
                    }
                } else if value.is_object() {
                    self.redact_value(value, obj_path.clone());
                }
            });
        }
    }

    /// redact all Value array values
    fn redact_value_array(&self, array: &mut Value) {
        array.as_array_mut().iter_mut().for_each(|values| {
            values.iter_mut().for_each(|val| {
                *val = Value::String(self.text_placeholder.to_string());
            });
        });
    }
}

#[cfg(test)]
mod test_redaction {

    use insta::assert_debug_snapshot;
    use serde_json::json;

    use super::*;

    #[test]
    fn can_redact_value_by_key() {
        let json = json!({
            "bar": "baz",
            "key": "value",
        })
        .to_string();

        let redact = Redact::default().add_keys(vec!["bar"]);

        assert_debug_snapshot!(redact.redact_str(&json));
    }

    #[test]
    fn can_redact_value_by_path() {
        let json = json!({
        "a": {
            "b": {
                "key": "redact_me",
            },
            "foo": "bar",
            "key": "skip-redaction"
        },
        "key": "skip-redaction"
            })
        .to_string();

        let redact = Redact::default().add_paths(vec!["a.foo"]);

        assert_debug_snapshot!(redact.redact_str(&json));
    }

    #[test]
    fn can_redact_value_by_prefix_path() {
        let json = json!({
        "a": {
            "b": {
                "key": "redact_me",
            },
            "foo": "bar",
            "key": "skip-redaction"
        },
        "key": "skip-redaction1"
            })
        .to_string();

        let redact = Redact::default().add_paths(vec!["a.*"]);

        assert_debug_snapshot!(redact.redact_str(&json));
    }

    #[test]
    fn can_redact_value_combination() {
        let json = json!({
        "foo": {
            "b": {
                "key": "redact_me",
            },
            "foo": "redact_me",
            "key": "redact_me",
        },
        "bar": {
            "b": {
                "key": "skip-redaction",
            },
            "foo": "skip-redaction",
            "key": "redact_me"
        },
        "key": "redact_me",
        "baz": "skip-redaction"
        })
        .to_string();

        let redact = Redact::default()
            .add_paths(vec!["foo.*", "bar.key"])
            .add_keys(vec!["key"]);

        assert_debug_snapshot!(redact.redact_str(&json));
    }

    #[test]
    fn can_redact_value_array() {
        let redact = Redact::default();

        let mut array_value = Value::Array(vec![
            serde_json::Value::String("value-1".to_string()),
            serde_json::Value::String("value-2".to_string()),
        ]);
        redact.redact_value_array(&mut array_value);
        assert_debug_snapshot!(array_value);
    }
}
