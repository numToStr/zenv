use std::ffi::OsString;
use std::path::PathBuf;
use std::process::exit;

pub struct Cli {
    // Print help information
    pub help: bool,

    // Path to .env file
    pub path: Option<PathBuf>,

    // Name of the command
    pub binary: Option<OsString>,

    // Arguments of the command
    pub bin_args: Vec<OsString>,
}

impl Cli {
    pub fn parse() -> Result<Cli, pico_args::Error> {
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
        let mut args = pico_args::Arguments::from_vec(args);
        let mut bin_args = bin_args.into_iter();
        let res = Cli {
            help: args.contains(["-h", "--help"]),
            path: args.opt_value_from_str(["-f", "--file"])?,
            binary: bin_args.next(),
            bin_args: bin_args.collect(),
        };

        // It's up to the caller what to do with the remaining arguments.
        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Unknown arguments: {:?}", remaining);
            exit(1)
        }

        Ok(res)
    }
}

#[macro_export]
macro_rules! assert_arg {
    ($val:expr, $flag:expr) => {
        match $val {
            Some(v) => v,
            _ => {
                eprintln!("ERROR:: {}", $flag);
                exit(1)
            }
        }
    };
}

#[macro_export]
macro_rules! assert_result {
    ($val:expr) => {
        match $val {
            Ok(v) => v,
            Err(err) => {
                eprintln!("ERROR:: {}", err);
                exit(1)
            }
        }
    };
}
