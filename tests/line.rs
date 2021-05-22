use zenv::*;

pub fn parse(line: &str) -> Option<(String, String)> {
    match Line::from(line) {
        Line::KeyVal(KeyVal { k, v, .. }) => Some((k, v)),
        _ => None,
    }
}

#[test]
fn basic() {
    let res = parse("BASIC=basic").unwrap();

    assert_eq!(res, ("BASIC".to_string(), "basic".to_string()))
}

#[test]
fn empty_line() {
    let res = Line::from("");

    assert_eq!(res, Line::Empty)
}

#[test]
fn empty_value() {
    let res = parse("EMPTY=").unwrap();

    assert_eq!(res, ("EMPTY".to_string(), "".to_string()))
}

#[test]
fn single_quotes() {
    let res = parse("SINGLE_QUOTES='single_quotes'").unwrap();

    assert_eq!(
        res,
        ("SINGLE_QUOTES".to_string(), "single_quotes".to_string())
    )
}

#[test]
fn single_quotes_spaced() {
    let res = parse("SINGLE_QUOTES_SPACED='    single_quotes_spaced    '").unwrap();

    assert_eq!(
        res,
        (
            "SINGLE_QUOTES_SPACED".to_string(),
            "    single_quotes_spaced    ".to_string()
        )
    )
}

#[test]
fn double_quotes() {
    let res = parse("DOUBLE_QUOTES=\"double_quotes\"").unwrap();

    assert_eq!(
        res,
        ("DOUBLE_QUOTES".to_string(), "double_quotes".to_string())
    )
}

#[test]
fn double_quotes_spaced() {
    let res = parse("DOUBLE_QUOTES_SPACED=\"    double_quotes_spaced    \"").unwrap();

    assert_eq!(
        res,
        (
            "DOUBLE_QUOTES_SPACED".to_string(),
            "    double_quotes_spaced    ".to_string()
        )
    )
}

#[test]
fn expand_newlines() {
    let res = parse(r#"EXPAND_NEWLINES="expand\nnew\nlines""#).unwrap();

    assert_eq!(
        res,
        (
            "EXPAND_NEWLINES".to_string(),
            "expand\nnew\nlines".to_string()
        )
    )
}

#[test]
fn escaped_newlines_dquote() {
    let res = parse(r#"ESCAPED_NEWLINES_DQUOTE="escaped\\nnew\\nlines""#).unwrap();

    assert_eq!(
        res,
        (
            "ESCAPED_NEWLINES_DQUOTE".to_string(),
            "escaped\\nnew\\nlines".to_string()
        )
    )
}

#[test]
fn escaped_newlines_squote() {
    let res = parse("ESCAPED_NEWLINES_SQUOTE='escaped\\nnew\\nlines'").unwrap();

    assert_eq!(
        res,
        (
            "ESCAPED_NEWLINES_SQUOTE".to_string(),
            "escaped\\nnew\\nlines".to_string()
        )
    )
}

#[test]
fn dont_expand_unquoted() {
    let res = parse("DONT_EXPAND_UNQUOTED=dont\nexpand\nnew\nlines").unwrap();

    assert_eq!(
        res,
        (
            "DONT_EXPAND_UNQUOTED".to_string(),
            "dont\\nexpand\\nnew\\nlines".to_string()
        )
    )
}

#[test]
fn dont_expand_squoted() {
    let res = parse("DONT_EXPAND_SQUOTED='dont\nexpand\nnew\nlines'").unwrap();

    assert_eq!(
        res,
        (
            "DONT_EXPAND_SQUOTED".to_string(),
            "dont\\nexpand\\nnew\\nlines".to_string()
        )
    )
}

#[test]
fn equal_signs() {
    let res = parse("EQUAL_SIGNS=equals==").unwrap();

    assert_eq!(res, ("EQUAL_SIGNS".to_string(), "equals==".to_string()))
}

#[test]
fn retain_inner_quotes() {
    let res = parse(r#"RETAIN_INNER_QUOTES={"foo": "bar"}"#).unwrap();

    assert_eq!(
        res,
        (
            "RETAIN_INNER_QUOTES".to_string(),
            r#"{"foo": "bar"}"#.to_string()
        )
    )
}

#[test]
fn retain_leading_dquote() {
    let res = parse("RETAIN_LEADING_DQUOTE=\"retained_dquote").unwrap();

    assert_eq!(
        res,
        (
            "RETAIN_LEADING_DQUOTE".to_string(),
            "\"retained_dquote".to_string()
        )
    )
}

#[test]
fn retain_leading_dquote_with_comment() {
    let res = parse("RETAIN_LEADING_DQUOTE_COMMENT=\"retained_dquote comment # comment").unwrap();

    assert_eq!(
        res,
        (
            "RETAIN_LEADING_DQUOTE_COMMENT".to_string(),
            "\"retained_dquote comment".to_string()
        )
    )
}

#[test]
fn retain_leading_squote() {
    let res = parse("RETAIN_LEADING_SQUOTE='retained_squote").unwrap();

    assert_eq!(
        res,
        (
            "RETAIN_LEADING_SQUOTE".to_string(),
            "'retained_squote".to_string()
        )
    )
}

#[test]
fn retain_leading_squote_with_comment() {
    let res = parse("RETAIN_LEADING_SQUOTE_COMMENT='retained_squote comment # comment").unwrap();

    assert_eq!(
        res,
        (
            "RETAIN_LEADING_SQUOTE_COMMENT".to_string(),
            "'retained_squote comment".to_string()
        )
    )
}

#[test]
fn retain_trailing_dquote() {
    let res = parse("RETAIN_TRAILING_DQUOTE=retained_dquote\"").unwrap();

    assert_eq!(
        res,
        (
            "RETAIN_TRAILING_DQUOTE".to_string(),
            "retained_dquote\"".to_string()
        )
    )
}

#[test]
fn retain_trailing_squote() {
    let res = parse("RETAIN_TRAILING_SQUOTE=retained_squote'").unwrap();

    assert_eq!(
        res,
        (
            "RETAIN_TRAILING_SQUOTE".to_string(),
            "retained_squote'".to_string()
        )
    )
}

#[test]
fn retain_inner_quotes_as_string() {
    let res = parse(r#"RETAIN_INNER_QUOTES_AS_STRING='{"foo": "bar"}'"#).unwrap();

    assert_eq!(
        res,
        (
            "RETAIN_INNER_QUOTES_AS_STRING".to_string(),
            r#"{"foo": "bar"}"#.to_string()
        )
    )
}

#[test]
fn trim_space_from_unquoted() {
    let res = parse("TRIM_SPACE_FROM_UNQUOTED=    some spaced out string").unwrap();

    assert_eq!(
        res,
        (
            "TRIM_SPACE_FROM_UNQUOTED".to_string(),
            "some spaced out string".to_string()
        )
    )
}

#[test]
fn just_space() {
    let res = Line::from("      ");

    assert_eq!(res, Line::Empty)
}

#[test]
fn spaced_key() {
    let res = parse("    SPACED_KEY = spaced_key").unwrap();

    assert_eq!(res, ("SPACED_KEY".to_string(), "spaced_key".to_string()))
}

#[test]
fn comment_basic() {
    let res = Line::from("# COMMENT=comment");

    assert_eq!(res, Line::Empty)
}

#[test]
fn comment_at_end_unquoted() {
    let res = parse("COMMENT_AT_END=comment_at_end # this is the comment").unwrap();

    assert_eq!(
        res,
        ("COMMENT_AT_END".to_string(), "comment_at_end".to_string())
    )
}

#[test]
fn comment_at_end_squote() {
    let res =
        parse("COMMENT_AT_END_SQUOTE='comment_at_##_end_squote_#' # this is the comment").unwrap();

    assert_eq!(
        res,
        (
            "COMMENT_AT_END_SQUOTE".to_string(),
            "comment_at_##_end_squote_#".to_string()
        )
    )
}

#[test]
fn comment_at_end_dquote() {
    let res =
        parse("COMMENT_AT_END_DQUOTE=\"comment_at_#_end_dquote\" # this is the comment").unwrap();

    assert_eq!(
        res,
        (
            "COMMENT_AT_END_DQUOTE".to_string(),
            "comment_at_#_end_dquote".to_string()
        )
    )
}
