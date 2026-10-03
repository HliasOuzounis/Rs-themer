use crate::modules::{file_handling, theme_changer};

use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn add(image_paths: Vec<PathBuf>, set_theme: bool) {
    let mut config_file = file_handling::get_config_file();

    let mut themes_map = file_handling::read_config_file(&config_file);

    for image in &image_paths{
        check_image_path(&image);

        let full_path = fs::canonicalize(image).unwrap();
        let theme_name = String::from(full_path.file_stem().unwrap().to_str().unwrap());

        if themes_map.contains_key(&theme_name){
            continue;
        }

        themes_map.insert(theme_name, full_path.to_str().unwrap().to_string());
    }

    // for theme in themes_map {
    //         config_file
    //             .write_fmt(format_args!("{}:{}\n", theme.0, theme.1))
    //             .expect("Error occured when writing to file");
    // }

    // 2. Open file with 'truncate' to overwrite the old duplicate-ridden data
    config_file = file_handling::get_empty_config_file();
    
    // 3. Write the deduplicated map back to disk
    for (name, path) in &themes_map {
        writeln!(config_file, "{}:{}", name, path).expect("Error writing to file");
    }

    if set_theme {
        if image_paths.len() > 1 {
            panic!("Cannot set theme when adding multiple images");
        }
        let full_path = fs::canonicalize(&image_paths[0]).unwrap();
        theme_changer::change_theme(&full_path.to_str().unwrap());
    }
}

fn check_image_path(image_path: &PathBuf) {
    if !image_path.exists() {
        panic!("No such image exists");
    }
    let extension = image_path.extension().unwrap().to_str().unwrap();
    if !["jpg", "png"].contains(&extension) {
        panic!("Invalid file format. Please use jpg or png");
    }
}
