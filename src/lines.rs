use std::collections::HashMap;

const B_SLASH: char = '\\';
const NLINE: char = '\n';
const QUOTE: char = '\'';
const D_QUOTE: char = '"';

#[derive(Debug, PartialEq)]
pub enum Line {
    KeyVal(String, String),
    Empty,
}

impl Line {
    fn replace_lf(line: &str) -> String {
        let mut s = String::with_capacity(line.len());
        let mut chars = line.chars();

        loop {
            match chars.next() {
                // If escape char is found
                Some(x) if x == B_SLASH => match chars.next() {
                    Some(n) if n == 'n' => {
                        s.push(NLINE);
                    }
                    Some(n) => {
                        s.push(x);
                        s.push(n);
                    }
                    None => s.push(x),
                },
                // chars() automagically converts \n into LF
                // no special handling of new line character
                Some(x) => s.push(x),
                _ => break,
            }
        }

        s
    }
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
                        let val = Self::replace_lf(v.trim_matches(D_QUOTE));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_key_val_test() {
        if let Line::KeyVal(key, val) = Line::from("HELLO=world") {
            assert_eq!(key, "HELLO");
            assert_eq!(val, "world");
        }
    }

    #[test]
    fn parse_line_only_key_test() {
        if let Line::KeyVal(key, val) = Line::from("HELLO=") {
            assert_eq!(key, "HELLO");
            assert_eq!(val, "");
        }
    }

    #[test]
    fn parse_line_spaces_trimmed_test() {
        if let Line::KeyVal(key, val) = Line::from("FOO= This is spaces ") {
            assert_eq!(key, "FOO");
            assert_eq!(val, "This is spaces");
        }
    }

    #[test]
    fn parse_line_single_double_quote_end_start_test() {
        if let Line::KeyVal(key, val) = Line::from("FOO='inside quote'") {
            assert_eq!(key, "FOO");
            assert_eq!(val, "inside quote");
        }
        if let Line::KeyVal(key2, val2) = Line::from(r#"FOO="inside double quote""#) {
            assert_eq!(key2, "FOO");
            assert_eq!(val2, "inside double quote");
        }
    }

    #[test]
    fn parse_line_spaces_preserved_test() {
        if let Line::KeyVal(key, val) = Line::from("FOO=' inside quote '") {
            assert_eq!(key, "FOO");
            assert_eq!(val, " inside quote ");
        }
        if let Line::KeyVal(key2, val2) = Line::from(r#"FOO=" inside double quote ""#) {
            assert_eq!(key2, "FOO");
            assert_eq!(val2, " inside double quote ");
        }
    }

    #[test]
    fn parse_line_json_test() {
        if let Line::KeyVal(key, val) = Line::from(r#"JSON={"foo": "bar"}"#) {
            assert_eq!(key, "JSON");
            assert_eq!(val, r#"{"foo": "bar"}"#);
        }
    }

    #[test]
    fn parse_line_commented_test() {
        assert_eq!(Line::from("# FOO=bar"), Line::Empty);
    }

    #[test]
    fn parse_line_empty_test() {
        assert_eq!(Line::from(""), Line::Empty);
    }

    #[test]
    fn parse_line_newline_char_test() {
        if let Line::KeyVal(key, val) = Line::from(r#"WHAT="You\nAre\nAwesome""#) {
            assert_eq!(key, "WHAT");
            assert_eq!(val, "You\nAre\nAwesome");
        }
    }

    #[test]
    fn parse_line_escaped_newline_char_test() {
        if let Line::KeyVal(key, val) = Line::from(r#"WHAT="You\\nAre\\nAwesome""#) {
            assert_eq!(key, "WHAT");
            assert_eq!(val, r#"You\\nAre\\nAwesome"#);
        }
    }

    #[test]
    fn parse_line_no_newline_char_test() {
        if let Line::KeyVal(key, val) = Line::from(r#"WHAT='You\nAre\nAwesome'"#) {
            assert_eq!(key, "WHAT");
            assert_eq!(val, "You\\nAre\\nAwesome");
        }
    }
}
