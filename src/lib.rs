mod parser;

use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

// Just re-exporting to use as a standalone parser
pub use parser::{KeyVal, Line, Lines, Quote};

#[derive(Debug)]
pub struct Zenv {
    path: PathBuf,
    expand: bool,
}

impl Zenv {
    pub fn new(path: &str, expand: bool) -> Self {
        Self {
            path: PathBuf::from(path),
            expand,
        }
    }

    pub fn parse(&self) -> Result<HashMap<String, String>> {
        let path = &self.path;

        if !path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Unable to find file - {}", path.display()),
            ));
        }

        let lines = {
            let r = read_to_string(&self.path)?;
            Lines::from(r)
        };

        let hash = match self.expand {
            true => lines.expand(),
            false => lines.to_hash_map(),
        };

        Ok(hash)
    }

    pub fn configure(&self) -> Result<()> {
        let vars = self.parse()?;

        for (key, val) in vars {
            std::env::set_var(key, val);
        }

        Ok(())
    }
}
