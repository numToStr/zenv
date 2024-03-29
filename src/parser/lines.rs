use std::{collections::HashMap, env};

use super::line::{KeyVal, Line, Quote};

/// `Lines` is used to parse the sequence of lines
///
/// [`Zenv`](crate::Zenv) is built on top it. And if you want you can directly use this but don't :)
#[derive(Debug)]
pub struct Lines<'l> {
    lines: Vec<KeyVal<'l>>,
}

impl<'l> From<&'l str> for Lines<'l> {
    fn from(lines: &'l str) -> Self {
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

impl<'l> Lines<'l> {
    /// Create `Lines` from a vec of [`Line`]. Can be useful if you manually parsing individual lines
    ///
    /// Example
    /// ```
    /// use zenv::{Lines, Line};
    ///
    /// let lines = vec![
    ///     Line::from("BASIC=basic"),
    ///     Line::Empty,
    ///     Line::from("QUOTED='quoted'")
    /// ];
    ///
    /// let parsed = Lines::new(lines).to_hash_map();
    ///
    /// assert_eq!(parsed.get("BASIC").unwrap(), &"basic".to_string());
    /// assert_eq!(parsed.get("QUOTED").unwrap(), &"quoted".to_string());
    /// ```
    pub fn new(lines: Vec<Line<'l>>) -> Self {
        let lines = lines
            .into_iter()
            .filter_map(|x| match x {
                Line::KeyVal(x) => Some(x),
                _ => None,
            })
            .collect();

        Self { lines }
    }

    /// Parses the lines and converts into a hashmap
    ///
    /// Example
    /// ```
    /// use zenv::Lines;
    ///
    /// const LINES: &str = r#"
    /// BASIC=basic
    /// QUOTED='quoted'
    /// "#;
    ///
    /// let parsed = Lines::from(LINES).to_hash_map();
    ///
    /// assert_eq!(parsed.get("BASIC").unwrap(), &"basic".to_string());
    /// assert_eq!(parsed.get("QUOTED").unwrap(), &"quoted".to_string());
    /// ```
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        let lines = &self.lines;
        let mut hash = HashMap::with_capacity(lines.len());

        for line in lines {
            hash.insert(line.k.to_string(), line.v.to_string());
        }

        hash
    }

    /// Parses the lines and does variable substitution then converts into a hashmap
    ///
    /// Example
    /// ```
    /// use zenv::Lines;
    ///
    /// const LINES: &str = r#"
    /// BASIC=basic
    /// EXPANDED="${BASIC}_is_expanded"
    /// "#;
    ///
    /// let parsed = Lines::from(LINES).expand();
    ///
    /// assert_eq!(parsed.get("BASIC").unwrap(), &"basic".to_string());
    /// assert_eq!(parsed.get("EXPANDED").unwrap(), &"basic_is_expanded".to_string());
    /// ```
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

                                    let k = x.to_string() + &key;

                                    (k, true)
                                }
                                _ => (String::with_capacity(0), false),
                            };

                            let found = match vars.get(&key) {
                                Some(x) => x.to_string(),
                                _ => env::var(&key).unwrap_or_default(),
                            };

                            new_val.push_str(&found);

                            if is_consumed {
                                // Need to find the terminator charactor
                                // Which is also consumed by the take_while() above
                                let idx = chars.clone().count();

                                // If we reach the end of the string
                                if idx == 0 {
                                    continue;
                                }

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
        }

        vars
    }
}
