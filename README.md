<h1 align="center">
    <code>.zenv</code>
</h1>
<p align="center">Dotenv (.env) loader written in rust 🦀</p>

<!-- TODO: badges -->

## ✨ Features

-   Fast as it is written in rust
-   Use as lib/crate or as a standalone cli
-   Support variable expansion

## 🚀 Installation

#### Crate

Add `zenv` with a version of your choice in the `Cargo.toml`

#### CLI

-   **Using `cargo`**

```sh
cargo install zenv --features=cli
```

-   **From binaries**

Check out the [Release page](https://github.com/numToStr/zenv/releases) for prebuild binaries for `zenv`, available for different operating systems.

## 🤞 Usage

#### Crate

```rust
use zenv::Zenv;
use std::path::PathBuf;

fn main() {
    Zenv::new(PathBuf::new(".env"), true).configure().ok();
}
```

Read full [documention](https:://docs.rs/zenv)

#### CLI

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
