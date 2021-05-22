use std::{collections::HashMap, ops::Add};

const LF: char = '\n';
const HASH: char = '#';
const B_SLASH: char = '\\';
const S_QUOTE: char = '\'';
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

                        Line::KeyVal(key, Self::retain_quote(v.to_string(), val))
                    }
                    Some(S_QUOTE) => {
                        let val: String = chars
                            .take_while(|x| x != &S_QUOTE)
                            .map(Self::escape_lf)
                            .collect();

                        Line::KeyVal(key, Self::retain_quote(v.to_string(), val))
                    }
                    Some(a) => {
                        let mut val = Self::escape_lf(a);

                        val.push_str(
                            &chars
                                .take_while(|x| x != &HASH)
                                .map(Self::escape_lf)
                                .collect::<String>(),
                        );

                        Line::KeyVal(key, val.trim().to_string())
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
        let lines: Vec<Line> = lines
            .lines()
            .into_iter()
            .filter_map(|x| match Line::from(x) {
                Line::Empty => None,
                x => Some(x),
            })
            .collect();

        Self { lines }
    }
}

impl Lines {
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        let lines = &self.lines;
        let mut hash = HashMap::with_capacity(lines.len());

        for line in lines {
            if let Line::KeyVal(k, v) = line {
                hash.insert(k.into(), v.into());
            }
        }

        hash
    }

    pub fn expand(&self) -> HashMap<String, String> {
        let mut vars = Self::to_hash_map(self);

        for line in &self.lines {
            if let Line::KeyVal(k, v) = line {
                let mut new_val = String::with_capacity(v.len());
                let mut chars = v.chars();

                loop {
                    match chars.next() {
                        Some('$') => {
                            let (key, is_consumed): (String, bool) = match chars.next() {
                                Some('{') => {
                                    (chars.by_ref().take_while(|c| c != &'}').collect(), false)
                                }
                                Some(x) => {
                                    let key: String = chars
                                        .by_ref()
                                        .take_while(|c| c.is_alphanumeric() || c == &'_')
                                        .collect();

                                    let mut x = x.to_string();

                                    x.push_str(&key);

                                    (x, true)
                                }
                                _ => (String::new(), false),
                            };

                            if let Some(found) = vars.get(&key) {
                                new_val.push_str(found);

                                if is_consumed {
                                    // Need to find the terminator charactor
                                    // Which is also consumed by the take_while() above
                                    let idx = chars.clone().count();

                                    // If we reach the end of the string
                                    if idx == 0 {
                                        continue;
                                    }

                                    if let Some(consumed) = v.chars().rev().skip(idx).take(1).next()
                                    {
                                        new_val.push(consumed);
                                    };
                                }
                            }
                        }
                        Some(a) => new_val.push(a),
                        _ => break,
                    }
                }

                vars.insert(k.to_string(), new_val);
            }
        }

        vars
    }
}
