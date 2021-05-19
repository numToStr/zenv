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

                        Line::KeyVal(key, format!("{}{}", a, val).trim().to_string())
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
