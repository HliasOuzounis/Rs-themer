use crate::{change_theme::change_theme, store};

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

const IMAGE_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

pub fn add(image_paths: Vec<PathBuf>, set_theme: bool) -> Result<()> {
    if set_theme && image_paths.len() > 1 {
        bail!("Cannot set theme when adding multiple images");
    }

    let mut saved = store::load()?;

    for image in &image_paths {
        let full_path = check_image_path(image)?;
        let theme_name = full_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .with_context(|| format!("{} has no valid UTF-8 file name", image.display()))?
            .to_string();

        if saved.themes.contains_key(&theme_name) {
            continue;
        }

        saved.themes.insert(theme_name, full_path);
    }

    store::save(&saved)?;

    if set_theme {
        change_theme(&fs::canonicalize(&image_paths[0])?)?;
    }
    Ok(())
}

/// Returns the absolute path if the image exists and has a supported extension.
fn check_image_path(image_path: &Path) -> Result<PathBuf> {
    let full_path = fs::canonicalize(image_path)
        .with_context(|| format!("Could not find {}", image_path.display()))?;

    let extension = full_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_lowercase)
        .unwrap_or_default();
    if !IMAGE_EXTENSIONS.contains(&extension.as_str()) {
        bail!(
            "{} is not a supported image. Use jpg, jpeg or png",
            image_path.display()
        );
    }
    Ok(full_path)
}
