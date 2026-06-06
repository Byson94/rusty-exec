use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The rust file to run
    pub file: PathBuf,

    /// Use a faster, but less limiting backend
    #[arg(short, long, global = true)]
    pub jit: bool,

    /// Print debug information
    #[arg(short, long, global = true)]
    pub debug: bool,
}
