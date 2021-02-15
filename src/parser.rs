const QUOTE: char = '\'';
const D_QUOTE: char = '"';

pub fn parse_line<'a>(line: &'a str) -> Option<(&'a str, &'a str)> {
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let mut parts = line.splitn(2, '=');

    match (parts.next(), parts.next()) {
        (Some(k), Some(v)) => {
            return match (
                v.starts_with(QUOTE),
                v.ends_with(QUOTE),
                v.starts_with(D_QUOTE),
                v.ends_with(D_QUOTE),
            ) {
                (true, true, false, false) => Some((k, v.trim_matches(QUOTE))),
                (false, false, true, true) => Some((k, v.trim_matches(D_QUOTE))),
                _ => Some((k, v.trim())),
            };
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
}
