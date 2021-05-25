use std::ops::Add;

const LF: char = '\n';
const HASH: char = '#';
const B_SLASH: char = '\\';
const S_QUOTE: char = '\'';
const D_QUOTE: char = '"';

/// Type of the quote
#[derive(Debug, PartialEq)]
pub enum Quote {
    /// When the value is single quoted i.e. `'`
    Single,

    /// When the value is double quoted i.e. `"`
    Double,

    /// When the value is not quoted
    No,
}

/// To collect the info about the current line
#[derive(Debug, PartialEq)]
pub struct KeyVal {
    /// `key` of the variable
    pub k: String,

    /// `value` of the variable
    pub v: String,

    /// Whether the value is quoted or not
    pub q: Quote,
}

/// (Can be) Used to parse the current line
///
/// Example
/// ```
/// use zenv::{Line, KeyVal, Quote};
///
/// let line = Line::from("BASIC=basic");
///
/// let k = "BASIC".to_string();
/// let v = "basic".to_string();
/// assert_eq!(line, Line::KeyVal(KeyVal { k, v, q: Quote::No }));
///
/// // Commented line
/// let empty = Line::from("# COMMENT=commented");
///
/// assert_eq!(empty, Line::Empty);
///
/// // With quotes
/// let quoted = Line::from("S_QUOTED='single_quoted'");
///
/// let k = "S_QUOTED".to_string();
/// let v = "single_quoted".to_string();
/// assert_eq!(quoted, Line::KeyVal(KeyVal { k, v, q: Quote::Single }));
/// ```
#[derive(Debug, PartialEq)]
pub enum Line {
    /// When the current line is a `key=val` pair
    KeyVal(KeyVal),

    /// When the current line is empty
    Empty,
}

impl Line {
    fn replace_lf(line: &str) -> String {
        let mut s = String::with_capacity(line.len());
        let mut chars = line.chars();

        loop {
            match chars.next() {
                // If \ is found
                Some(x) if x == B_SLASH => match chars.next() {
                    // "\n" -> Chars: ['\\', 'n']
                    Some('n') => {
                        s.push(LF);
                    }
                    // "\\n" -> Chars: ['\\', '\\', 'n']
                    Some(B_SLASH) => {
                        s.push(B_SLASH);
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

    fn escape_lf(x: char) -> String {
        if x == LF {
            x.escape_debug().to_string()
        } else {
            x.to_string()
        }
    }

    fn retain_quote(orgnl: String, after: String, q: Quote) -> (String, Quote) {
        // If both strings length matches then it is not closed
        if orgnl.len().eq(&after.len().add(1)) {
            let new_val: String = orgnl.chars().take_while(|c| c != &HASH).collect();

            (new_val.trim().to_string(), Quote::No)
        } else {
            (after, q)
        }
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

                        let (v, q) = Self::retain_quote(v.to_string(), val, Quote::Double);

                        Line::KeyVal(KeyVal { k: key, v, q })
                    }
                    Some(S_QUOTE) => {
                        let val: String = chars
                            .take_while(|x| x != &S_QUOTE)
                            .map(Self::escape_lf)
                            .collect();

                        let (v, q) = Self::retain_quote(v.to_string(), val, Quote::Single);

                        Line::KeyVal(KeyVal { k: key, v, q })
                    }
                    Some(a) => {
                        let mut val = Self::escape_lf(a);

                        val.push_str(
                            &chars
                                .take_while(|x| x != &HASH)
                                .map(Self::escape_lf)
                                .collect::<String>(),
                        );

                        Line::KeyVal(KeyVal {
                            k: key,
                            v: val.trim().to_string(),
                            q: Quote::No,
                        })
                    }
                    _ => Line::KeyVal(KeyVal {
                        k: key,
                        v: String::with_capacity(0),
                        q: Quote::No,
                    }),
                }
            }
            _ => Self::Empty,
        }
    }
}
