use crate::info::{DESC, NAME, VERSION};
use lexopt::{
    Arg::{Long, Short, Value},
    Parser,
};
use std::{ffi::OsString, process};

pub struct Cli {
    // Whether to substitute variables or not
    pub expand: bool,
    // Path to .env file
    pub path: String,
    // Name of the command
    pub binary: Option<OsString>,
    // Arguments of the command
    pub args: Vec<OsString>,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            expand: false,
            path: ".env".to_string(),
            binary: None,
            args: vec![],
        }
    }
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
                    cli.path = parser.value()?.into_string()?;
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
    -f, --file          Path to env file [default: .env]

ARGS:
    <binary>            Command that needs to be executed
    [args]...           Arguments for the command

Examples:
    {name} -- node index.js
    {name} -f .env.dev -- npm run dev
    {name} -f .env.prod -- terraform apply
",
            name = NAME,
            ver = VERSION,
            desc = DESC,
        )
    }
}
