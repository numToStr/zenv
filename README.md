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

-   Fast as it is written in rust
-   Use as lib/crate or as a standalone cli
-   Support variable expansion

## 🚀 Installation

### Crate

Add [`zenv`](https://crates.io/crates/zenv) with a version of your choice in the `Cargo.toml`

```toml
[dependencies]
zenv = "<version>" # Make sure it's the latest version
```

### CLI

-   **Using `cargo`**

```bash
cargo install zenv --features=cli
```

-   **From binaries**

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
    zenv [FLAGS] [OPTIONS] -- <binary> [args]...

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
    zenv -f .env -- node index.js
    zenv -f .env -- npm run dev
    zenv -f .env -- terraform apply
```

<!-- TODO: explanation -->

## 🙏 Credits

-   [motdotla/dotenv](https://github.com/motdotla/dotenv) for Javascript
-   [joho/godotenv](https://github.com/joho/godotenv) for Go
-   [bkeepers/dotenv](https://github.com/bkeepers/dotenv) for Ruby
