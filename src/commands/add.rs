use crate::{change_theme::change_theme, store};

use std::fs;
use std::path::{Path, PathBuf};

pub fn add(image_paths: Vec<PathBuf>, set_theme: bool) {
    let mut saved = store::load();

    for image in &image_paths {
        check_image_path(image);

        let full_path = fs::canonicalize(image).unwrap();
        let theme_name = String::from(full_path.file_stem().unwrap().to_str().unwrap());

        if saved.themes.contains_key(&theme_name) {
            continue;
        }

        saved.themes.insert(theme_name, full_path);
    }

    store::save(&saved);

    if set_theme {
        if image_paths.len() > 1 {
            panic!("Cannot set theme when adding multiple images");
        }
        let full_path = fs::canonicalize(&image_paths[0]).unwrap();
        change_theme(&full_path);
    }
}

fn check_image_path(image_path: &Path) {
    if !image_path.exists() {
        panic!("No such image exists");
    }
    let extension = image_path.extension().unwrap().to_str().unwrap();
    if !["jpg", "png"].contains(&extension) {
        panic!("Invalid file format. Please use jpg or png");
    }
}
