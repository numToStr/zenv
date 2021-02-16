const QUOTE: char = '\'';
const D_QUOTE: char = '"';
const N_LINE: &'static str = "\\n";

pub fn parse_line<'a>(line: &'a str) -> Option<(String, String)> {
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let mut parts = line.splitn(2, '=');

    match (parts.next(), parts.next()) {
        (Some(k), Some(v)) => {
            let key = k.trim().to_string();

            let first = v.chars().next();
            let last = v.chars().next_back();

            match (first, last) {
                (Some(D_QUOTE), Some(D_QUOTE)) => {
                    let val = if v.contains(N_LINE) {
                        v.trim_matches(D_QUOTE).replace(N_LINE, "\n")
                    } else {
                        v.trim_matches(D_QUOTE).to_string()
                    };

                    Some((key, val))
                }
                (Some(QUOTE), Some(QUOTE)) => Some((key, v.trim_matches(QUOTE).into())),
                _ => Some((key, v.trim().to_string())),
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_key_val_test() {
        let (key, val) = parse_line("HELLO=world").unwrap();
        assert_eq!(key, "HELLO");
        assert_eq!(val, "world");
    }

    #[test]
    fn parse_line_only_key_test() {
        let (key, val) = parse_line("HELLO=").unwrap();
        assert_eq!(key, "HELLO");
        assert_eq!(val, "");
    }

    #[test]
    fn parse_line_spaces_trimmed_test() {
        let (key, val) = parse_line("FOO= This is spaces ").unwrap();
        assert_eq!(key, "FOO");
        assert_eq!(val, "This is spaces");
    }

    #[test]
    fn parse_line_single_double_quote_end_start_test() {
        let (key, val) = parse_line("FOO='inside quote'").unwrap();
        let (key2, val2) = parse_line(r#"FOO="inside double quote""#).unwrap();
        assert_eq!(key, "FOO");
        assert_eq!(val, "inside quote");
        assert_eq!(key2, "FOO");
        assert_eq!(val2, "inside double quote");
    }

    #[test]
    fn parse_line_spaces_preserved_test() {
        let (key, val) = parse_line("FOO=' inside quote '").unwrap();
        let (key2, val2) = parse_line(r#"FOO=" inside double quote ""#).unwrap();
        assert_eq!(key, "FOO");
        assert_eq!(val, " inside quote ");
        assert_eq!(key2, "FOO");
        assert_eq!(val2, " inside double quote ");
    }

    #[test]
    fn parse_line_json_test() {
        let (key, val) = parse_line(r#"JSON={"foo": "bar"}"#).unwrap();
        assert_eq!(key, "JSON");
        assert_eq!(val, r#"{"foo": "bar"}"#);
    }

    #[test]
    fn parse_line_commented_test() {
        let empty = parse_line("# FOO=bar");
        assert_eq!(empty, None);
    }

    #[test]
    fn parse_line_empty_test() {
        let empty = parse_line("");
        assert_eq!(empty, None);
    }

    #[test]
    fn parse_line_newline_char_test() {
        let (key, val) = parse_line(r#"WHAT="You\nAre\nAwesome""#).unwrap();
        assert_eq!(key, "WHAT");
        assert_eq!(val, "You\nAre\nAwesome");
    }

    #[test]
    fn parse_line_no_newline_char_test() {
        let (key, val) = parse_line(r#"WHAT='You\nAre\nAwesome'"#).unwrap();
        assert_eq!(key, "WHAT");
        assert_eq!(val, "You\\nAre\\nAwesome");
    }
}
