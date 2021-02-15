mod parser;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::PathBuf,
};

type KeyHash = HashMap<String, String>;
type VarsBuf = Lines<BufReader<File>>;

// Just re-exporting to use as a standalone parser
pub use parser::parse_line;

#[derive(Debug)]
pub struct Denv {
    path: PathBuf,
}

impl Denv {
    pub fn new(path: PathBuf) -> Denv {
        Denv { path }
    }

    pub fn read(&self) -> Result<VarsBuf> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);

        Ok(reader.lines())
    }

    pub fn parse(&self, lines: VarsBuf) -> KeyHash {
        let mut hash: KeyHash = HashMap::with_capacity(lines.size_hint().0);

        for line in lines {
            let line = line.expect("Unable to parse line");
            let p = parse_line(&line);

            if let Some((key, val)) = p {
                hash.entry(key.into()).or_insert(val.into());
            }
        }

        hash
    }

    pub fn config(&self) -> Result<()> {
        let lines = self.read()?;
        let vars = self.parse(lines);

        for (key, val) in vars {
            std::env::set_var(key, val);
        }

        Ok(())
    }
}
