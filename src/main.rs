use std::io::Result;

use denv::Denv;

pub fn main() -> Result<()> {
    Denv::new(".env".into()).config()
}
