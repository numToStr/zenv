mod parser;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::PathBuf,
};

// Just re-exporting to use as a standalone parser
pub use parser::*;

#[derive(Debug)]
pub struct Denv {
    path: PathBuf,
}

impl Denv {
    pub fn new(path: PathBuf) -> Denv {
        Denv { path }
    }

    fn read(&self) -> Result<Vec<String>> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        Ok(lines)
    }

    pub fn config(&self) -> Result<()> {
        let lines = self.read()?;
        let vars = parse_multi_line(&lines);

        for (key, val) in vars {
            std::env::set_var(key, val);
        }

        Ok(())
    }
}
