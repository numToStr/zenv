use std::process::{Command, Stdio};
use std::{ffi::OsString, path::PathBuf, process::exit};

use denv::Denv;

const HELP: &str = "\
denv - Dotenv (.env) loader written in rust

USAGE:
  denv [FLAGS] [OPTIONS] -- <binary> [args]...

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  -f, --file            Path to .env file

ARGS:
    <binary>            Command that needs to be executed
    [args]...           Arguments for the command

Examples:
    denv -f .env -- node index.js
    denv -f .env -- npm run dev
    denv -f .env -- terraform apply
";

#[derive(Debug)]
struct Args {
    help: bool,
    path: Option<PathBuf>,
    bin_args: Vec<OsString>,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    // `from_vec` takes `OsString`, not `String`.
    let mut args: Vec<_> = std::env::args_os().collect();
    args.remove(0); // remove the executable path.

    // Find and process `--`.
    let bin_args = if let Some(dash_dash) = args.iter().position(|arg| arg == "--") {
        // Store all arguments following ...
        let later_args = args.drain(dash_dash + 1..).collect();
        // .. then remove the `--`
        args.pop();
        later_args
    } else {
        Vec::with_capacity(0)
    };

    // Now pass the remaining arguments through to `pico_args`.
    let mut args = pico_args::Arguments::from_vec(args);
    let res = Args {
        help: args.contains(["-h", "--help"]),
        path: args.opt_value_from_str(["-f", "--file"])?,
        bin_args,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = args.finish();
    if !remaining.is_empty() {
        eprintln!("Unknown arguments: {:?}", remaining);
        exit(1)
    }

    Ok(res)
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };

    if args.help {
        print!("{}", HELP);
        exit(0)
    }

    let fpath = match args.path {
        Some(path) => path,
        _ => {
            eprintln!("-f/--file option is required");
            exit(1)
        }
    };

    let mut a = args.bin_args.into_iter();

    let code = match a.next() {
        Some(binary) => {
            let denv = Denv::new(fpath);
            let vars = denv.parse().expect("Unable to parse variables");

            let exit_status = Command::new(&binary)
                .args(a)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .envs(vars)
                .spawn()
                .expect(&format!(
                    "Unable to spawn program: {}",
                    binary.to_str().unwrap()
                ))
                .wait()
                .expect("Failed to grab exit code");

            exit_status.code().unwrap_or(1)
        }
        _ => {
            eprintln!("<bin> is required");
            1
        }
    };

    exit(code)
}
