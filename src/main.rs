mod cli;
mod info;

use cli::Cli;
use std::process::{exit, Command, Stdio};
use zenv::Zenv;

fn bootstrap() -> Result<i32, lexopt::Error> {
    let cli = Cli::parse()?;

    let path = cli.path.ok_or("-f/--file is required")?;
    let bin = cli.binary.ok_or("<binary> name is required")?;

    let vars = Zenv::new(&path, cli.expand)
        .parse()
        .map_err(|e| e.to_string())?;

    let mut program = Command::new(&bin)
        .args(&cli.args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .envs(vars)
        .spawn()
        .map_err(|_| "Unable to spawn program!")?;

    let code = program
        .wait()
        .map_err(|e| e.to_string())?
        .code()
        .ok_or("Failed to grab the exit code!")?;

    Ok(code)
}

fn main() {
    match bootstrap() {
        Ok(code) => exit(code),
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };
}
