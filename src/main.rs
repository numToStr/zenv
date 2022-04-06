mod cli;
mod info;

use cli::Cli;
use std::process::{exit, Command, Stdio};
use zenv::Zenv;

fn bootstrap() -> Result<i32, lexopt::Error> {
    let cli = Cli::parse()?;

    let cmd = cli.command.ok_or("<command> name is required")?;

    let vars = Zenv::new(&cli.path, cli.expand)
        .parse()
        .map_err(|e| e.to_string())?;

    let mut program = Command::new(&cmd)
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
