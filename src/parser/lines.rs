use std::collections::HashMap;

use super::line::{KeyVal, Line, Quote};

#[derive(Debug)]
pub struct Lines {
    lines: Vec<KeyVal>,
}

impl From<String> for Lines {
    fn from(lines: String) -> Self {
        let lines: Vec<KeyVal> = lines
            .lines()
            .into_iter()
            .filter_map(|x| match Line::from(x) {
                Line::KeyVal(x) => Some(x),
                _ => None,
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
            hash.insert(line.k.to_string(), line.v.to_string());
        }

        hash
    }

    pub fn expand(&self) -> HashMap<String, String> {
        let mut vars = Self::to_hash_map(self);

        for line in &self.lines {
            if let KeyVal {
                q: Quote::Double,
                k,
                v,
            } = line
            {
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
