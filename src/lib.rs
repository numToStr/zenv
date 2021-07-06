//! Zenv is a dotenv loader which parses and loads your environment variables from a `.env` file.
//!
//! This crate also supports variable substitution inside your .env file, if not found then
//! tries to fetch from the running operating system. By default, this is disabled.
//!
//! _This crate only meant to use inside a development environment._
//!
//! Example
//! ```
//! use zenv::{zenv, Zenv};
//!
//! fn main() {
//!     Zenv::new(".env", false).configure().ok();
//!     // is equivalent to
//!     zenv!();
//!
//!     // with other file
//!     zenv!(".env.development");
//!
//!     // or with variable substitution
//!     zenv!(".env.development", true);
//! }
//! ```

mod parser;

use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

// Just re-exporting to use as a standalone parser
pub use parser::{KeyVal, Line, Lines, Quote};

/// Use this to load and configure the environment variables
#[derive(Debug)]
pub struct Zenv {
    path: PathBuf,
    expand: bool,
}

impl Zenv {
    /// Create a new instance of Zenv with the provided file path
    pub fn new(path: &str, expand: bool) -> Self {
        Self {
            path: PathBuf::from(path),
            expand,
        }
    }

    /// Read and parse the file from provided path and returns a hashmap
    ///
    /// Example
    /// ```
    /// let parsed = zenv::Zenv::new("tests/.env.basic", false).parse().unwrap();
    ///
    /// assert_eq!(parsed.get("BASIC"), Some(&"basic".to_string()))
    /// ```
    pub fn parse(&self) -> Result<HashMap<String, String>> {
        let path = &self.path;

        if !path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Unable to find file - {}", path.display()),
            ));
        }

        let r = read_to_string(path)?;
        let lines = Lines::from(r.as_str());

        let hash = match self.expand {
            true => lines.expand(),
            false => lines.to_hash_map(),
        };

        Ok(hash)
    }

    /// Parse the file using [Zenv::parse] and sets the environment variable
    ///
    /// Example
    /// ```
    /// zenv::Zenv::new("tests/.env.basic", false).configure().ok();
    ///
    /// assert_eq!(std::env::var_os("BASIC"), Some("basic".into()))
    /// ```
    pub fn configure(&self) -> Result<()> {
        let vars = self.parse()?;

        for (key, val) in vars {
            std::env::set_var(key, val);
        }

        Ok(())
    }
}

/// This macro can be used as a shortcut for [`Zenv`]
///
/// Example
/// ```
/// use zenv::zenv;
///
/// zenv!();
///
/// // with other file
/// zenv!(".env.development");
///
/// // or with variable substitution
/// zenv!(".env.development", true);
/// ````
#[macro_export]
macro_rules! zenv {
    () => {
        zenv::Zenv::new(".env", false).configure().ok()
    };
    ($path:expr) => {
        zenv::Zenv::new($path, false).configure().ok()
    };
    ($path:expr, $expand:expr) => {
        zenv::Zenv::new($path, $expand).configure().ok()
    };
}
