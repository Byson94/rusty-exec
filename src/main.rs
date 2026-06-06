mod opts;
mod jit;
mod rustc;

use clap::Parser;
use log::Level;

fn main() {
    let args = opts::Args::parse();

    // set debug
    set_debug_levels(args.debug);
    if args.debug {
        log::info!("Debug logging enabled");
    }

    // ensure it exists
    if !args.file.exists() {
        log::error!("Given file does not exist");
        std::process::exit(1);
    }

    // pass work to backend
    if args.jit {
        if let Err(e) = jit::execute_jit(args.file) {
            log::error!("Failed to execute JIT: {e}");
            std::process::exit(1);
        }
    } else {
        if let Err(e) = rustc::execute_compile(args.file) {
            log::error!("Failed to compile: {e}");
            std::process::exit(1);
        }
    }
}

fn set_debug_levels(debug_mode: bool) {
    let mut builder = env_logger::Builder::from_default_env();

    if debug_mode {
        builder
            .filter_level(log::LevelFilter::Debug)
            .format_timestamp_secs()
            .format_module_path(true)
            .format_level(true);
    } else {
        builder
            .format(|buf, record| {
                use std::io::Write;

                match record.level() {
                    Level::Warn => writeln!(buf, "[WARN] {}", record.args()),
                    Level::Error => writeln!(buf, "[ERROR] {}", record.args()),
                    _ => writeln!(buf, "{}", record.args()),
                }
            })
            .filter_level(log::LevelFilter::Info);
    }

    builder.init();
}

