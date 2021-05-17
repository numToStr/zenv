use std::ffi::OsString;
use std::path::PathBuf;

use pico_args::Arguments;

use crate::info::{DESC, NAME, VERSION};

pub struct Cli {
    // Print help information
    pub help: bool,

    // Whether to substitute variables or not
    pub expand: bool,

    // Path to .env file
    path: Option<PathBuf>,

    // Name of the command
    binary: Option<OsString>,

    // Arguments of the command
    pub bin_args: Vec<OsString>,
}

impl Cli {
    pub fn parse() -> Result<Cli, String> {
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
            Vec::new()
        };

        // Now pass the remaining arguments through to `pico_args`.
        let mut args = Arguments::from_vec(args);
        let mut bin_args = bin_args.into_iter();
        let res = Cli {
            help: args.contains(["-h", "--help"]),
            expand: args.contains(["-x", "--expand"]),
            path: args
                .opt_value_from_str(["-f", "--file"])
                .map_err(|e| e.to_string())?,
            binary: bin_args.next(),
            bin_args: bin_args.collect(),
        };

        // It's up to the caller what to do with the remaining arguments.
        let remaining = args.finish();
        if !remaining.is_empty() {
            return Err(format!("Unknown arguments: {:?}", remaining));
        }

        Ok(res)
    }

    pub fn path(&self) -> Result<&PathBuf, &str> {
        self.path.as_ref().ok_or("-f/--file option is required")
    }

    pub fn binary(&self) -> Result<&OsString, &str> {
        self.binary.as_ref().ok_or("<binary> name is required")
    }

    pub fn help_doc() -> String {
        format!(
            "\
{name} {ver}
{desc}

USAGE:
    zenv [FLAGS] [OPTIONS] -- <binary> [args]...

FLAGS:
    -h, --help          Prints help information
    -x, --expand        Enable variable expansion

OPTIONS:
    -f, --file          Path to .env file

ARGS:
    <binary>            Command that needs to be executed
    [args]...           Arguments for the command

Examples:
    {name} -f .env -- node index.js
    {name} -f .env -- npm run dev
    {name} -f .env -- terraform apply
",
            name = NAME,
            ver = VERSION,
            desc = DESC,
        )
    }
}
