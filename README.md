<h1 align="center">
    <code>.zenv</code>
</h1>
<p align="center"><b>Dotenv (.env) loader written in rust 🦀</b></p>

<p align="center">
  <a aria-label="build" href="https://github.com/numToStr/zenv/actions/workflows/build.yml">
    <img alt="build" src="https://github.com/numToStr/zenv/actions/workflows/build.yml/badge.svg">
  </a>
  <a aria-label="docs" href="https://docs.rs/zenv">
    <img alt="docs" src="https://docs.rs/zenv/badge.svg">
  </a>
  <a aria-label="crates.io" href="https://crates.io/crates/zenv">
    <img alt="crates.io" src="https://img.shields.io/crates/v/zenv.svg">
  </a>
</p>

## ✨ Features

- Fast as it is written in rust
- Use as lib/crate or as a standalone cli
- Support variable expansion

## 🚀 Installation

### Crate

Add [`zenv`](https://crates.io/crates/zenv) with a version of your choice in the `Cargo.toml`

```toml
[dependencies]
zenv = "<version>" # Make sure it's the latest version
```

### CLI

- **Using `cargo`**

```bash
cargo install zenv --features=cli
```

- **Arch Linux**

```bash
# Using `yay`
yay -S zenv

# Using `pamac`
pamac build zenv
```

- **From binaries**

Check out the [Release page](https://github.com/numToStr/zenv/releases) for prebuild binaries for `zenv`, available for different operating systems.

## 🤞 Usage

### Crate

```rust
fn main() {
    zenv::Zenv::new(".env", false).configure().ok();

    // or use macro, which expands to above statement

    zenv::zenv!()
}
```

> Read the full [documention](https:://docs.rs/zenv)

### CLI

```
zenv
Dotenv (.env) loader written in rust

USAGE:
    zenv [FLAGS] [OPTIONS] -- <command> [args]...

FLAGS:
    -v, --version       Prints version
    -h, --help          Prints help information
    -x, --expand        Enable variable expansion

OPTIONS:
    -f, --file          Path to .env file

ARGS:
    <command>            Command that needs to be executed
    [args]...           Arguments for the command

Examples:
    zenv -f .env -- node index.js
    zenv -f .env -- npm run dev
    zenv -f .env -- terraform apply
```

## 🙌 Good to Know

### Basic

```bash
PORT=5000
NODE_ENV=production

# Single and double quotes are also supported
S_QUOTE='single_quoted'
D_QUOTE="double_quoted"
```

### Comments

Comments can be added by using `#` character.

```bash
# COMMENTED=commented
AT_THE_END=comment_at_the_end # I am here

# If you want # in you value then wrap the value in single or double quotes
QUOTED="quote_#_quoted" # I'll be removed
```

### New Line and Escaping

New lines can added by new line (`\n`) character and this only works if the values is surrounded by double quotes.

```bash
PRIVATE_KEY="-----BEGIN RSA PRIVATE KEY-----\nadflhsdlfsjkldfjklsdjf\n-----END RSA PRIVATE KEY-----"

# or like this
PRIVATE_KEY="-----BEGIN RSA PRIVATE KEY-----
adflhsdlfsjkldfjklsdjf
asdffwejdjf983283lk
-----END RSA PRIVATE KEY-----"
```

If you want to escape the new line character you can use the escape (`\`)

```bash
ESCAPED="escaped\\nnew\\nline"
```

### Substitution

`Zenv` also supports variable substitution (off by default) from the current file or from the operating system. Substitution only works if the values is double quoted ie.e `"` and can be achieved by the following:

- Using `${VAR}` pattern (recommended)
- Starting the variable name by `$` character, which terminates after reaching a character which is not `_` or alphanumeric.

```bash
BASIC=basic
EXPANDED='${BASIC}_expanded' # expands to 'basic_expanded'

# System variables (assuming `PATH` is available)
SYSTEM_VARIABLE="${PATH},/this/is/new/path"
```

## 🙏 Credits

- [motdotla/dotenv](https://github.com/motdotla/dotenv) (Javascript)
- [joho/godotenv](https://github.com/joho/godotenv) (Golang)
- [bkeepers/dotenv](https://github.com/bkeepers/dotenv) (Ruby)
