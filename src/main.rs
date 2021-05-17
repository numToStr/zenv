mod cli;
mod info;
use std::process::exit;
use std::process::{Command, Stdio};

use cli::Cli;
use zenv::Zenv;

use crate::info::{NAME, VERSION};

fn bootstrap() -> Result<i32, String> {
    let args = Cli::parse()?;

    if args.version {
        println!("{} {}", NAME, VERSION);
        return Ok(0);
    }

    if args.help {
        print!("{}", Cli::help_doc());
        return Ok(0);
    }

    let fpath = args.path()?;

    let binary = args.binary()?;

    let vars = Zenv::new(fpath.to_owned(), args.expand)
        .parse()
        .map_err(|e| e.to_string())?;

    let mut program = Command::new(&binary)
        .args(&args.bin_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .envs(vars)
        .spawn()
        .map_err(|_| format!("Unable to spawn program - `{}`", binary.to_str().unwrap()))?;

    let code = program
        .wait()
        .map_err(|e| e.to_string())?
        .code()
        .ok_or_else(|| "Failed to grab the exit code".to_string())?;

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
