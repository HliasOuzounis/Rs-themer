use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

pub fn change_theme(image_path: &Path) -> Result<()> {
    let output = match Command::new("wallust").arg("run").arg(image_path).output() {
        Err(err) if err.kind() == ErrorKind::NotFound => {
            bail!("wallust is not installed or not in PATH")
        }
        result => result.context("Could not run wallust")?,
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("wallust failed ({}):\n{}", output.status, stderr.trim_end());
    }
    Ok(())
}
