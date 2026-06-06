use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The rust file to run
    pub file: PathBuf,

    /// List of modules
    pub modules: Option<Vec<PathBuf>>,

    /// Print debug information
    #[arg(short, long, global = true)]
    pub debug: bool,
}
