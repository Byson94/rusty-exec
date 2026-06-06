use std::fs;
use syn::parse_file;
use std::path::PathBuf;
use anyhow::Result;

pub fn execute_jit(file: PathBuf) -> Result<()> {
    let code = fs::read_to_string(&file)?;

    // check to ensure that its valid
    // run `cargo check` in memory.

    let file = parse_file(&code).unwrap();

    for item in file.items {
        if let syn::Item::Fn(f) = item {
            println!("Found function: {}", f.sig.ident);
        }
    }

    Ok(())
}
