const QUOTE: char = '\'';
const D_QUOTE: char = '"';

pub struct LineParser;

impl<'a> LineParser {
    fn replacer(line: &'a str, builder: &mut String, to: &'a str) {
        let (again, rhs) = match line.find("\\n") {
            Some(pos) => {
                // New line can be at the start of the string
                let is_escaped = match pos.checked_sub(1) {
                    Some(idx) => match line.chars().nth(idx) {
                        Some('\\') => true,
                        _ => false,
                    },
                    _ => false,
                };

                match is_escaped {
                    true => {
                        let lhs: String = line.chars().take(pos + 2).collect();
                        builder.push_str(&lhs);
                    }
                    _ => {
                        let lhs: String = line.chars().take(pos).collect();
                        builder.push_str(&lhs);
                        builder.push_str(to);
                    }
                };

                let rhs = &line[(pos + 2)..];

                (true, rhs)
            }
            _ => (false, line),
        };

        if again {
            Self::replacer(&rhs, builder, to)
        } else {
            builder.push_str(&rhs)
        }
    }

    pub fn replace_new_line(line: &'a str) -> String {
        let mut builder = String::with_capacity(line.len());

        Self::replacer(line, &mut builder, "\n");

        builder
    }

    pub fn parse_line(line: &'a str) -> Option<(String, String)> {
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        let mut parts = line.splitn(2, '=');

        match (parts.next(), parts.next()) {
            (Some(k), Some(v)) => {
                let key = k.trim().to_string();
                let mut chars = v.chars();

                let first = chars.next();
                let last = chars.next_back();

                match (first, last) {
                    (Some(D_QUOTE), Some(D_QUOTE)) => {
                        let val = Self::replace_new_line(v.trim_matches(D_QUOTE));

                        Some((key, val))
                    }
                    (Some(QUOTE), Some(QUOTE)) => Some((key, v.trim_matches(QUOTE).into())),
                    _ => Some((key, v.trim().to_string())),
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_key_val_test() {
        let (key, val) = LineParser::parse_line("HELLO=world").unwrap();
        assert_eq!(key, "HELLO");
        assert_eq!(val, "world");
    }

    #[test]
    fn parse_line_only_key_test() {
        let (key, val) = LineParser::parse_line("HELLO=").unwrap();
        assert_eq!(key, "HELLO");
        assert_eq!(val, "");
    }

    #[test]
    fn parse_line_spaces_trimmed_test() {
        let (key, val) = LineParser::parse_line("FOO= This is spaces ").unwrap();
        assert_eq!(key, "FOO");
        assert_eq!(val, "This is spaces");
    }

    #[test]
    fn parse_line_single_double_quote_end_start_test() {
        let (key, val) = LineParser::parse_line("FOO='inside quote'").unwrap();
        let (key2, val2) = LineParser::parse_line(r#"FOO="inside double quote""#).unwrap();
        assert_eq!(key, "FOO");
        assert_eq!(val, "inside quote");
        assert_eq!(key2, "FOO");
        assert_eq!(val2, "inside double quote");
    }

    #[test]
    fn parse_line_spaces_preserved_test() {
        let (key, val) = LineParser::parse_line("FOO=' inside quote '").unwrap();
        let (key2, val2) = LineParser::parse_line(r#"FOO=" inside double quote ""#).unwrap();
        assert_eq!(key, "FOO");
        assert_eq!(val, " inside quote ");
        assert_eq!(key2, "FOO");
        assert_eq!(val2, " inside double quote ");
    }

    #[test]
    fn parse_line_json_test() {
        let (key, val) = LineParser::parse_line(r#"JSON={"foo": "bar"}"#).unwrap();
        assert_eq!(key, "JSON");
        assert_eq!(val, r#"{"foo": "bar"}"#);
    }

    #[test]
    fn parse_line_commented_test() {
        let empty = LineParser::parse_line("# FOO=bar");
        assert_eq!(empty, None);
    }

    #[test]
    fn parse_line_empty_test() {
        let empty = LineParser::parse_line("");
        assert_eq!(empty, None);
    }

    #[test]
    fn parse_line_newline_char_test() {
        let (key, val) = LineParser::parse_line(r#"WHAT="You\nAre\nAwesome""#).unwrap();
        assert_eq!(key, "WHAT");
        assert_eq!(val, "You\nAre\nAwesome");
    }

    #[test]
    fn parse_line_escaped_newline_char_test() {
        let (key, val) = LineParser::parse_line(r#"WHAT="You\\nAre\\nAwesome""#).unwrap();
        assert_eq!(key, "WHAT");
        assert_eq!(val, r#"You\\nAre\\nAwesome"#);
    }

    #[test]
    fn parse_line_no_newline_char_test() {
        let (key, val) = LineParser::parse_line(r#"WHAT='You\nAre\nAwesome'"#).unwrap();
        assert_eq!(key, "WHAT");
        assert_eq!(val, "You\\nAre\\nAwesome");
    }
}
