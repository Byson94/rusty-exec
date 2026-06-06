use std::path::PathBuf;
use std::process::{Command, Stdio};
use anyhow::{Result, Context, bail};
use std::fs;

pub fn execute_compile(file: PathBuf) -> Result<()> {
    let dir = tempfile::tempdir().context("Failed to get tempdir")?;
    let status = Command::new("cargo")
        .arg("init")
        .args(["--name", "quick-compile"])
        .current_dir(&dir)
        .status()?;

    if !status.success() {
        bail!("Failed to initialize rust workspace.");
    }

    let target_code = dir.path().join("src/main.rs");
    fs::copy(file, target_code)?;

    let status = Command::new("cargo")
        .arg("run")
        .current_dir(&dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        bail!("Failed to compile and run binary.")
    }

    Ok(())
}
