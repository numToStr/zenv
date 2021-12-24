use crate::info::{DESC, NAME, VERSION};
use lexopt::{
    Arg::{Long, Short, Value},
    Parser,
};
use std::{ffi::OsString, process};

#[derive(Default)]
pub struct Cli {
    // Whether to substitute variables or not
    pub expand: bool,
    // Path to .env file
    pub path: Option<String>,
    // Name of the command
    pub binary: Option<OsString>,
    // Arguments of the command
    pub args: Vec<OsString>,
}

impl Cli {
    pub fn parse() -> Result<Self, lexopt::Error> {
        let mut cli = Self::default();

        let mut parser = Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Short('v') | Long("version") => {
                    println!("{} {}", NAME, VERSION);
                    process::exit(0);
                }
                Short('h') | Long("help") => {
                    print!("{}", Self::help_doc());
                    process::exit(0);
                }
                Short('x') | Long("expand") => cli.expand = true,
                Short('f') | Long("file") => {
                    cli.path = parser.value()?.into_string().ok();
                }
                Value(val) => {
                    if cli.binary == None {
                        cli.binary = Some(val);
                    } else {
                        cli.args.push(val);
                    }
                }
                _ => return Err(arg.unexpected()),
            }
        }

        Ok(cli)
    }

    pub fn help_doc() -> String {
        format!(
            "\
{name} {ver}
{desc}

USAGE:
    {name} [FLAGS] [OPTIONS] -- <binary> [args]...

FLAGS:
    -v, --version       Prints version
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
