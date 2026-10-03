use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Contents of `themes.toml`. A BTreeMap keeps the file sorted by theme name.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    #[serde(default)]
    pub themes: BTreeMap<String, PathBuf>,
}

pub fn load() -> Store {
    let path = themes_file();
    match fs::read_to_string(&path) {
        Ok(content) => toml::from_str(&content)
            .unwrap_or_else(|err| fail(&format!("Could not parse {}:\n{err}", path.display()))),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Store::default(),
        Err(err) => fail(&format!("Could not read {}: {err}", path.display())),
    }
}

/// Write to a temporary file and rename it over the old one,
/// so a crash never leaves a half-written themes file.
pub fn save(store: &Store) {
    let path = themes_file();
    let dir = path.parent().unwrap();
    fs::create_dir_all(dir)
        .unwrap_or_else(|err| fail(&format!("Could not create {}: {err}", dir.display())));

    let content = toml::to_string(store)
        .unwrap_or_else(|err| fail(&format!("Could not serialize themes: {err}")));
    let tmp = path.with_extension("toml.tmp");
    fs::write(&tmp, content)
        .and_then(|_| fs::rename(&tmp, &path))
        .unwrap_or_else(|err| fail(&format!("Could not write {}: {err}", path.display())));
}

/// Image wallust last ran on, if any.
pub fn get_current_theme() -> Option<PathBuf> {
    let file_path = xdg_dir("XDG_CACHE_HOME", ".cache").join("wallust/wallpaper");
    let content = fs::read_to_string(file_path).ok()?;
    Some(PathBuf::from(content.trim_end()))
}

fn themes_file() -> PathBuf {
    xdg_dir("XDG_CONFIG_HOME", ".config").join("wlr/themes.toml")
}

/// `$VAR`, or `$HOME/<fallback>` when it's unset or empty (per the XDG spec).
fn xdg_dir(var: &str, fallback: &str) -> PathBuf {
    match env::var_os(var) {
        Some(dir) if !dir.is_empty() => PathBuf::from(dir),
        _ => {
            let home = env::var_os("HOME").unwrap_or_else(|| fail("HOME is not set"));
            PathBuf::from(home).join(fallback)
        }
    }
}

fn fail(msg: &str) -> ! {
    eprintln!("Error: {msg}");
    std::process::exit(1);
}
