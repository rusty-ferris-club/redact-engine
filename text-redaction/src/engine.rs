use regex::Regex;

use crate::data::Position;

/// Check if the given regex match to the given string
pub fn try_capture(
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

#[cfg(test)]
mod test_engine {

    use std::env;

    use insta::assert_debug_snapshot;

    use super::*;

    const TEXT: &str = "foo,bar,baz,foo";

    #[test]
    fn can_try_capture() {
        assert_debug_snapshot!(try_capture(TEXT, &Regex::new("(foo)").unwrap(), 1, false));
    }

    #[test]
    fn can_try_capture_with_info() {
        assert_debug_snapshot!(try_capture(TEXT, &Regex::new("(foo)").unwrap(), 1, true));
    }
}
