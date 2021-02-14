use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::PathBuf,
};

type KeyHash = HashMap<String, String>;
type VarLines = Lines<BufReader<File>>;

#[derive(Debug)]
pub struct Denv {
    path: PathBuf,
}

impl Denv {
    pub fn new(path: PathBuf) -> Denv {
        Denv { path }
    }

    pub fn read(&self) -> Result<VarLines> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);

        Ok(reader.lines())
    }

    pub fn parse_line<'a>(&self, line: &'a str) -> Option<(&'a str, &'a str)> {
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        let mut parts = line.splitn(2, '=');

        match (parts.next(), parts.next()) {
            (Some(k), Some(v)) => Some((k, v)),
            _ => None,
        }
    }

    pub fn parse(&self, lines: VarLines) -> Result<KeyHash> {
        let mut vars_hash: KeyHash = HashMap::new();

        for line in lines {
            let line = line?;

            let p = self.parse_line(&line);

            if let Some((key, val)) = p {
                vars_hash.entry(key.into()).or_insert(val.into());
            }
        }

        Ok(vars_hash)
    }

    pub fn config(&self) -> Result<()> {
        let lines = self.read()?;
        let vars = self.parse(lines)?;

        for (key, val) in vars {
            std::env::set_var(key, val);
        }

        Ok(())
    }
}
