use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Contents of `themes.toml`. A BTreeMap keeps the file sorted by theme name.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    #[serde(default)]
    pub themes: BTreeMap<String, PathBuf>,
}

pub fn load() -> Result<Store> {
    let path = themes_file()?;
    match fs::read_to_string(&path) {
        Ok(content) => {
            toml::from_str(&content).with_context(|| format!("Could not parse {}", path.display()))
        }
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(Store::default()),
        Err(err) => Err(err).with_context(|| format!("Could not read {}", path.display())),
    }
}

/// Write to a temporary file and rename it over the old one,
/// so a crash never leaves a half-written themes file.
pub fn save(store: &Store) -> Result<()> {
    let path = themes_file()?;
    let dir = path.parent().unwrap();
    fs::create_dir_all(dir).with_context(|| format!("Could not create {}", dir.display()))?;

    let content = toml::to_string(store).context("Could not serialize themes")?;
    let tmp = path.with_extension("toml.tmp");
    fs::write(&tmp, content)
        .and_then(|_| fs::rename(&tmp, &path))
        .with_context(|| format!("Could not write {}", path.display()))
}

/// Image wallust last ran on, if any.
pub fn get_current_theme() -> Option<PathBuf> {
    let file_path = xdg_dir("XDG_CACHE_HOME", ".cache")
        .ok()?
        .join("wallust/wallpaper");
    let content = fs::read_to_string(file_path).ok()?;
    Some(PathBuf::from(content.trim_end()))
}

fn themes_file() -> Result<PathBuf> {
    Ok(xdg_dir("XDG_CONFIG_HOME", ".config")?.join("wlr/themes.toml"))
}

/// `$VAR`, or `$HOME/<fallback>` when it's unset or empty (per the XDG spec).
fn xdg_dir(var: &str, fallback: &str) -> Result<PathBuf> {
    match env::var_os(var) {
        Some(dir) if !dir.is_empty() => Ok(PathBuf::from(dir)),
        _ => {
            let home =
                env::var_os("HOME").with_context(|| format!("Neither {var} nor HOME is set"))?;
            Ok(PathBuf::from(home).join(fallback))
        }
    }
}
