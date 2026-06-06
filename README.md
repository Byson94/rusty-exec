# rusty-exec

A simple, fast, local, tool for testing out things quickly in rust.

## Why?

If you wanted to test something in rust, you would have to:

- Create rust workspace with `cargo init`.
- Write your code in `main.rs`.
- Build.
- Run.
- Delete the directory.

This issue can be solved by using the rust playground, but it requires an internet connection.

## Installation

You can use one of these methods to install the tool: 

1. Install with cargo

```bash
cargo install --git https://github.com/Byson94/rusty-exec
```

2. Clone and build manually

```
git clone https://github.com/Byson94/rusty-exec
cargo build --release
```

## Usage

Execute a file:

```bash
rusty-exec example.rs
```

Execute a file along with modules:

```bash
# Both files and directories can be passed as modules
# They can be included like so:
# 
# mod module;
rusty-exec example.rs foo.rs baz/
```
