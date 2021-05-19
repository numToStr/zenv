use std::collections::HashMap;

const B_SLASH: char = '\\';
const NLINE: char = '\n';
const QUOTE: char = '\'';
const D_QUOTE: char = '"';
const HASH: char = '#';

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
        if line.is_empty() || line.starts_with(HASH) {
            return Self::Empty;
        };

        let mut parts = line.splitn(2, '=');

        match (parts.next(), parts.next()) {
            (Some(k), Some(v)) => {
                let key = k.trim().to_string();
                let mut chars = v.chars();

                let first = chars.next();

                match first {
                    Some(D_QUOTE) => {
                        let val = {
                            let v: String = chars.take_while(|x| x != &D_QUOTE).collect();
                            Self::replace_lf(&v)
                        };

                        Line::KeyVal(key, val)
                    }
                    Some(QUOTE) => {
                        let val: String = chars.take_while(|x| x != &QUOTE).collect();

                        Line::KeyVal(key, val)
                    }
                    Some(a) => {
                        let val: String = chars.take_while(|x| x != &HASH).collect();

                        Line::KeyVal(key, format!("{}{}", a, val.trim()).trim().to_string())
                    }
                    _ => Line::KeyVal(key, String::new()),
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
        let lines: Vec<Line> = lines.lines().into_iter().map(Line::from).collect();

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

    pub fn expand(self) -> HashMap<String, String> {
        let mut vars = Self::into_hash_map(self);
        let cloned = vars.clone();

        for (k, v) in cloned {
            let mut new_val = String::with_capacity(v.len());
            let mut v_chars = v.chars();

            loop {
                match v_chars.next() {
                    Some('$') => {
                        let x: String = v_chars
                            .by_ref()
                            .take_while(|c| c.is_alphanumeric() || c == &'_')
                            .collect();

                        if let Some(found) = vars.get(&x) {
                            new_val.push_str(found);

                            // Need to find the terminator charactor
                            // Which is also consumed by the take_while() above
                            let idx = v_chars.clone().count();
                            if let Some(consumed) = v.chars().rev().skip(idx).take(1).next() {
                                new_val.push(consumed);
                            };
                        }
                    }
                    Some(a) => new_val.push(a),
                    _ => break,
                }
            }

            vars.insert(k.to_string(), new_val);
        }

        vars
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
