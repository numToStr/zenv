use std::io::Result;

use denv::Denv;

pub fn main() -> Result<()> {
    let denv = Denv::new(".env".into());
    // println!("{:#?}", denv.parse(denv.read()?)?);
    // Ok(())
    denv.config()
}
