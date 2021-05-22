use std::ops::Add;

const LF: char = '\n';
const HASH: char = '#';
const B_SLASH: char = '\\';
const S_QUOTE: char = '\'';
const D_QUOTE: char = '"';

#[derive(Debug, PartialEq)]
pub enum Quote {
    Single,
    Double,
    No,
}

#[derive(Debug, PartialEq)]
pub struct KeyVal {
    pub k: String,
    pub v: String,
    pub q: Quote,
}

#[derive(Debug, PartialEq)]
pub enum Line {
    KeyVal(KeyVal),
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

    fn retain_quote(orgnl: String, after: String) -> String {
        // If both strings length matches then it is not closed
        if orgnl.len().eq(&after.len().add(1)) {
            let new_val: String = orgnl.chars().take_while(|c| c != &HASH).collect();

            new_val.trim().to_string()
        } else {
            after
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

                        Line::KeyVal(KeyVal {
                            k: key,
                            v: Self::retain_quote(v.to_string(), val),
                            q: Quote::Double,
                        })
                    }
                    Some(S_QUOTE) => {
                        let val: String = chars
                            .take_while(|x| x != &S_QUOTE)
                            .map(Self::escape_lf)
                            .collect();

                        Line::KeyVal(KeyVal {
                            k: key,
                            v: Self::retain_quote(v.to_string(), val),
                            q: Quote::Single,
                        })
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
