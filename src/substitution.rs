use std::collections::HashMap;

pub struct Substitution<'a> {
    vars: &'a mut HashMap<String, String>,
}

impl<'a> Substitution<'a> {
    pub fn new(vars: &'a mut HashMap<String, String>) -> Self {
        Self { vars }
    }

    fn get_position(&self, value: &str) -> (usize, usize) {
        let mut chars = value.chars();

        match chars.position(|x| x == '$') {
            Some(pos) => {
                let start = pos + 1;
                let count = match chars.position(|c| !c.is_alphanumeric() && c != '_') {
                    Some(x) => x,
                    None => value.len() - start,
                };

                (start, count)
            }
            None => (0, 0),
        }
    }

    fn build_value(&self, val: String) -> String {
        let new_val = match self.get_position(&val) {
            (a, b) if a == 0 && b == 0 => val,
            (start, count) => {
                let key: String = val.chars().skip(start).take(count).collect();

                let found = match self.vars.get(&key) {
                    Some(found) => found.to_string(),
                    None => String::new(),
                };

                let rhs = &val[0..(start - 1)];
                let lhs = &val[(start + count)..];

                format!("{}{}{}", rhs, found, lhs)
            }
        };

        if new_val.contains('$') {
            self.build_value(new_val)
        } else {
            new_val
        }
    }

    pub fn substitute(&mut self) {
        for (key, val) in self.vars.clone() {
            let new_val = self.build_value(val);
            self.vars.insert(key, new_val);
        }
    }
}
