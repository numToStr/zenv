use std::collections::HashMap;

type KeyHash = HashMap<String, String>;

pub fn parse_line<'a>(line: &'a str) -> Option<(&'a str, &'a str)> {
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let mut parts = line.splitn(2, '=');

    match (parts.next(), parts.next()) {
        (Some(k), Some(v)) => Some((k, v)),
        _ => None,
    }
}

pub fn parse_multi_line<'a>(lines: &'a Vec<String>) -> KeyHash {
    let mut hash: KeyHash = HashMap::with_capacity(lines.len());

    for line in lines {
        let p = parse_line(&line);

        if let Some((key, val)) = p {
            hash.entry(key.into()).or_insert(val.into());
        }
    }

    hash
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
    fn parse_multi_line_test() {
        let vars = parse_multi_line(&vec![
            "NODE_ENV=production".into(),
            "".into(),
            "WHAT=".into(),
            "# COMMENTED=this will be hidden".into(),
            "SOMETHING=That I don' know".into(),
        ]);

        assert_eq!(vars.len(), 3);
    }
}
