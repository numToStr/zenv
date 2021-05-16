use crate::replacer::{replace_new_line, NLINE};
use std::collections::HashMap;

const QUOTE: char = '\'';
const D_QUOTE: char = '"';

#[derive(Debug)]
pub enum Line {
    KeyVal(String, String),
    Empty,
}

impl From<&str> for Line {
    fn from(line: &str) -> Self {
        if line.is_empty() || line.starts_with('#') {
            return Self::Empty;
        };

        let mut parts = line.splitn(2, '=');

        match (parts.next(), parts.next()) {
            (Some(k), Some(v)) => {
                let key = k.trim().to_string();
                let mut chars = v.chars();

                let first = chars.next();
                let last = chars.next_back();

                match (first, last) {
                    (Some(D_QUOTE), Some(D_QUOTE)) => {
                        let val = replace_new_line(v.trim_matches(D_QUOTE));

                        Line::KeyVal(key, val)
                    }
                    (Some(QUOTE), Some(QUOTE)) => Line::KeyVal(key, v.trim_matches(QUOTE).into()),
                    _ => Line::KeyVal(key, v.trim().to_string()),
                }
            }
            _ => Self::Empty,
        }
    }
}

#[derive(Debug)]
pub struct Lines {
    lines: Vec<Line>,
}

impl From<String> for Lines {
    fn from(lines: String) -> Self {
        let lines: Vec<Line> = lines.split(NLINE).into_iter().map(Line::from).collect();

        Self { lines }
    }
}

impl Lines {
    pub fn into_hash_map(self) -> HashMap<String, String> {
        let lines = self.lines;
        let mut hash = HashMap::with_capacity(lines.len());

        for line in lines {
            if let Line::KeyVal(k, v) = line {
                hash.insert(k, v);
            }
        }

        hash
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn parse_line_key_val_test() {
//         let (key, val) = Line::from("HELLO=world");
//         assert_eq!(key, "HELLO");
//         assert_eq!(val, "world");
//     }
//
//     #[test]
//     fn parse_line_only_key_test() {
//         let (key, val) = Line::from("HELLO=").unwrap();
//         assert_eq!(key, "HELLO");
//         assert_eq!(val, "");
//     }
//
//     #[test]
//     fn parse_line_spaces_trimmed_test() {
//         let (key, val) = Line::from("FOO= This is spaces ").unwrap();
//         assert_eq!(key, "FOO");
//         assert_eq!(val, "This is spaces");
//     }
//
//     #[test]
//     fn parse_line_single_double_quote_end_start_test() {
//         let (key, val) = Line::from("FOO='inside quote'").unwrap();
//         let (key2, val2) = Line::from(r#"FOO="inside double quote""#).unwrap();
//         assert_eq!(key, "FOO");
//         assert_eq!(val, "inside quote");
//         assert_eq!(key2, "FOO");
//         assert_eq!(val2, "inside double quote");
//     }
//
//     #[test]
//     fn parse_line_spaces_preserved_test() {
//         let (key, val) = Line::from("FOO=' inside quote '").unwrap();
//         let (key2, val2) = Line::from(r#"FOO=" inside double quote ""#).unwrap();
//         assert_eq!(key, "FOO");
//         assert_eq!(val, " inside quote ");
//         assert_eq!(key2, "FOO");
//         assert_eq!(val2, " inside double quote ");
//     }
//
//     #[test]
//     fn parse_line_json_test() {
//         let (key, val) = Line::from(r#"JSON={"foo": "bar"}"#).unwrap();
//         assert_eq!(key, "JSON");
//         assert_eq!(val, r#"{"foo": "bar"}"#);
//     }
//
//     #[test]
//     fn parse_line_commented_test() {
//         let empty = Line::from("# FOO=bar");
//         assert_eq!(empty, None);
//     }
//
//     #[test]
//     fn parse_line_empty_test() {
//         let empty = Line::from("");
//         assert_eq!(empty, None);
//     }
//
//     #[test]
//     fn parse_line_newline_char_test() {
//         let (key, val) = Line::from(r#"WHAT="You\nAre\nAwesome""#).unwrap();
//         assert_eq!(key, "WHAT");
//         assert_eq!(val, "You\nAre\nAwesome");
//     }
//
//     #[test]
//     fn parse_line_escaped_newline_char_test() {
//         let (key, val) = Line::from(r#"WHAT="You\\nAre\\nAwesome""#).unwrap();
//         assert_eq!(key, "WHAT");
//         assert_eq!(val, r#"You\\nAre\\nAwesome"#);
//     }
//
//     #[test]
//     fn parse_line_no_newline_char_test() {
//         let (key, val) = Line::from(r#"WHAT='You\nAre\nAwesome'"#).unwrap();
//         assert_eq!(key, "WHAT");
//         assert_eq!(val, "You\\nAre\\nAwesome");
//     }
// }
