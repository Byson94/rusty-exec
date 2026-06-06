use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use anyhow::{Result, Context, bail};
use std::fs;

pub fn execute_compile(file: PathBuf, modules: Option<Vec<PathBuf>>) -> Result<()> {
    let dir = tempfile::tempdir().context("Failed to get tempdir")?;
    let filename = file.file_name().context("Filename not found")?.to_os_string();
    let filestem = file.file_stem().context("Filestem not found")?.to_os_string();

    let target_code = dir.path().join(&filename);
    fs::copy(file, target_code)?;

    if let Some(modules) = modules {
        for module in modules {
            if !module.exists() {
                log::error!("Module {} does not exist.", &module.display());
                continue;
            }

            if fs::metadata(&module)?.is_dir() {
                let target_dir = dir.path().join(&module);
                fs::create_dir_all(&target_dir)?;

                if let Ok(entries) = fs::read_dir(&module) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let file_name = entry.file_name();
                            let destination = target_dir.join(file_name);

                            fs::copy(entry.path(), destination)?;
                        }
                    }
                }
            } else {
                let target = dir.path().join(&module);
                fs::copy(module, target)?;
            }
        }
    }

    list_paths(&dir.path())?;

    let status = Command::new("rustc")
        .arg(&filename)
        .arg("--crate-type=bin")
        .current_dir(&dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        bail!("Failed to compile.");
    }

    let binary = dir.path().join(filestem);
    let status = Command::new(binary)
        .arg("--crate-type=bin")
        .current_dir(&dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        bail!("Failed to run.");
    }

    Ok(())
}

fn list_paths(dir: &Path) -> Result<()> {
    let paths = fs::read_dir(&dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if fs::metadata(&path)?.is_dir() {
            list_paths(&path)?;
        } else {
            log::debug!("Name: {}", path.display())
        }
    }

    Ok(())
}
