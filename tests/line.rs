use zenv::*;

#[test]
fn basic() {
    let res = Line::from("BASIC=basic");

    assert_eq!(res, Line::KeyVal("BASIC".to_string(), "basic".to_string()))
}

#[test]
fn empty_line() {
    let res = Line::from("");

    assert_eq!(res, Line::Empty)
}

#[test]
fn empty_value() {
    let res = Line::from("EMPTY=");

    assert_eq!(res, Line::KeyVal("EMPTY".to_string(), "".to_string()))
}

#[test]
fn single_quotes() {
    let res = Line::from("SINGLE_QUOTES='single_quotes'");

    assert_eq!(
        res,
        Line::KeyVal("SINGLE_QUOTES".to_string(), "single_quotes".to_string())
    )
}

#[test]
fn single_quotes_spaced() {
    let res = Line::from("SINGLE_QUOTES_SPACED='    single_quotes_spaced    '");

    assert_eq!(
        res,
        Line::KeyVal(
            "SINGLE_QUOTES_SPACED".to_string(),
            "    single_quotes_spaced    ".to_string()
        )
    )
}

#[test]
fn double_quotes() {
    let res = Line::from("DOUBLE_QUOTES=\"double_quotes\"");

    assert_eq!(
        res,
        Line::KeyVal("DOUBLE_QUOTES".to_string(), "double_quotes".to_string())
    )
}

#[test]
fn double_quotes_spaced() {
    let res = Line::from("DOUBLE_QUOTES_SPACED=\"    double_quotes_spaced    \"");

    assert_eq!(
        res,
        Line::KeyVal(
            "DOUBLE_QUOTES_SPACED".to_string(),
            "    double_quotes_spaced    ".to_string()
        )
    )
}

#[test]
fn expand_newlines() {
    let res = Line::from(r#"EXPAND_NEWLINES="expand\nnew\nlines""#);

    assert_eq!(
        res,
        Line::KeyVal(
            "EXPAND_NEWLINES".to_string(),
            "expand\nnew\nlines".to_string()
        )
    )
}

#[test]
fn escaped_newlines_dquote() {
    let res = Line::from(r#"ESCAPED_NEWLINES_DQUOTE="escaped\\nnew\\nlines""#);

    assert_eq!(
        res,
        Line::KeyVal(
            "ESCAPED_NEWLINES_DQUOTE".to_string(),
            "escaped\\nnew\\nlines".to_string()
        )
    )
}

#[test]
fn escaped_newlines_squote() {
    let res = Line::from("ESCAPED_NEWLINES_SQUOTE='escaped\\nnew\\nlines'");

    assert_eq!(
        res,
        Line::KeyVal(
            "ESCAPED_NEWLINES_SQUOTE".to_string(),
            "escaped\\nnew\\nlines".to_string()
        )
    )
}

#[test]
fn dont_expand_unquoted() {
    let res = Line::from("DONT_EXPAND_UNQUOTED=dont\nexpand\nnew\nlines");

    assert_eq!(
        res,
        Line::KeyVal(
            "DONT_EXPAND_UNQUOTED".to_string(),
            "dont\\nexpand\\nnew\\nlines".to_string()
        )
    )
}

#[test]
fn dont_expand_squoted() {
    let res = Line::from("DONT_EXPAND_SQUOTED='dont\nexpand\nnew\nlines'");

    assert_eq!(
        res,
        Line::KeyVal(
            "DONT_EXPAND_SQUOTED".to_string(),
            "dont\\nexpand\\nnew\\nlines".to_string()
        )
    )
}

#[test]
fn equal_signs() {
    let res = Line::from("EQUAL_SIGNS=equals==");

    assert_eq!(
        res,
        Line::KeyVal("EQUAL_SIGNS".to_string(), "equals==".to_string())
    )
}

#[test]
fn retain_inner_quotes() {
    let res = Line::from(r#"RETAIN_INNER_QUOTES={"foo": "bar"}"#);

    assert_eq!(
        res,
        Line::KeyVal(
            "RETAIN_INNER_QUOTES".to_string(),
            r#"{"foo": "bar"}"#.to_string()
        )
    )
}

#[test]
fn retain_leading_dquote() {
    let res = Line::from("RETAIN_LEADING_DQUOTE=\"retained_dquote");

    assert_eq!(
        res,
        Line::KeyVal(
            "RETAIN_LEADING_DQUOTE".to_string(),
            "\"retained_dquote".to_string()
        )
    )
}

#[test]
fn retain_leading_dquote_with_comment() {
    let res = Line::from("RETAIN_LEADING_DQUOTE_COMMENT=\"retained_dquote comment # comment");

    assert_eq!(
        res,
        Line::KeyVal(
            "RETAIN_LEADING_DQUOTE_COMMENT".to_string(),
            "\"retained_dquote comment".to_string()
        )
    )
}

#[test]
fn retain_leading_squote() {
    let res = Line::from("RETAIN_LEADING_SQUOTE='retained_squote");

    assert_eq!(
        res,
        Line::KeyVal(
            "RETAIN_LEADING_SQUOTE".to_string(),
            "'retained_squote".to_string()
        )
    )
}

#[test]
fn retain_leading_squote_with_comment() {
    let res = Line::from("RETAIN_LEADING_SQUOTE_COMMENT='retained_squote comment # comment");

    assert_eq!(
        res,
        Line::KeyVal(
            "RETAIN_LEADING_SQUOTE_COMMENT".to_string(),
            "'retained_squote comment".to_string()
        )
    )
}

#[test]
fn retain_trailing_dquote() {
    let res = Line::from("RETAIN_TRAILING_DQUOTE=retained_dquote\"");

    assert_eq!(
        res,
        Line::KeyVal(
            "RETAIN_TRAILING_DQUOTE".to_string(),
            "retained_dquote\"".to_string()
        )
    )
}

#[test]
fn retain_trailing_squote() {
    let res = Line::from("RETAIN_TRAILING_SQUOTE=retained_squote'");

    assert_eq!(
        res,
        Line::KeyVal(
            "RETAIN_TRAILING_SQUOTE".to_string(),
            "retained_squote'".to_string()
        )
    )
}

#[test]
fn retain_inner_quotes_as_string() {
    let res = Line::from(r#"RETAIN_INNER_QUOTES_AS_STRING='{"foo": "bar"}'"#);

    assert_eq!(
        res,
        Line::KeyVal(
            "RETAIN_INNER_QUOTES_AS_STRING".to_string(),
            r#"{"foo": "bar"}"#.to_string()
        )
    )
}

#[test]
fn trim_space_from_unquoted() {
    let res = Line::from("TRIM_SPACE_FROM_UNQUOTED=    some spaced out string");

    assert_eq!(
        res,
        Line::KeyVal(
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
    let res = Line::from("    SPACED_KEY = spaced_key");

    assert_eq!(
        res,
        Line::KeyVal("SPACED_KEY".to_string(), "spaced_key".to_string())
    )
}

#[test]
fn comment_basic() {
    let res = Line::from("# COMMENT=comment");

    assert_eq!(res, Line::Empty)
}

#[test]
fn comment_at_end_unquoted() {
    let res = Line::from("COMMENT_AT_END=comment_at_end # this is the comment");

    assert_eq!(
        res,
        Line::KeyVal("COMMENT_AT_END".to_string(), "comment_at_end".to_string())
    )
}

#[test]
fn comment_at_end_squote() {
    let res =
        Line::from("COMMENT_AT_END_SQUOTE='comment_at_##_end_squote_#' # this is the comment");

    assert_eq!(
        res,
        Line::KeyVal(
            "COMMENT_AT_END_SQUOTE".to_string(),
            "comment_at_##_end_squote_#".to_string()
        )
    )
}

#[test]
fn comment_at_end_dquote() {
    let res = Line::from("COMMENT_AT_END_DQUOTE=\"comment_at_#_end_dquote\" # this is the comment");

    assert_eq!(
        res,
        Line::KeyVal(
            "COMMENT_AT_END_DQUOTE".to_string(),
            "comment_at_#_end_dquote".to_string()
        )
    )
}
