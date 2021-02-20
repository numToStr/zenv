mod cli;
use std::process::exit;
use std::process::{Command, Stdio};

use cli::Cli;
use denv::Denv;

const HELP: &'static str = "\
denv - Dotenv (.env) loader written in rust

USAGE:
  denv [FLAGS] [OPTIONS] -- <binary> [args]...

FLAGS:
  -h, --help            Prints help information
  -x, --expand          Enable variable expansion

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

fn main() {
    let args = assert_result!(Cli::parse());

    if args.help {
        print!("{}", HELP);
        exit(0)
    }

    let fpath = assert_arg!(args.path, "-f/--file option is required");

    let binary = assert_arg!(args.binary, "<binary> name is required");

    let vars = assert_result!(Denv::new(fpath, args.expand).parse());

    // for (key, val) in &vars {
    //     println!("{} {}", key, val);
    // }

    let mut program = assert_result!(Command::new(&binary)
        .args(args.bin_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .envs(vars)
        .spawn()
        .map_err(|_| format!("Unable to spawn program - `{}`", binary.to_str().unwrap())));

    let code = {
        let exit_status = assert_result!(program.wait().map_err(|_| "Failed to grab exit code"));
        exit_status.code().unwrap_or(1)
    };

    exit(code)
}
