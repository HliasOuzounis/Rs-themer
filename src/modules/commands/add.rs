use crate::modules::file_handling;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn add(image_paths: Vec<PathBuf>) {
    let mut config_file = file_handling::get_config_file();

    let mut existing_themes = file_handling::read_config_file(&config_file);

    for image in image_paths{
        check_image_path(&image);

        let full_path = fs::canonicalize(image).unwrap();
        let theme_name = String::from(full_path.file_stem().unwrap().to_str().unwrap());

        if existing_themes.contains_key(&theme_name){
            continue;
        }

        existing_themes.insert(theme_name, full_path.to_str().unwrap().to_string());
    }

    for theme in existing_themes {
            config_file
                .write_fmt(format_args!("{}:{}\n", theme.0, theme.1))
                .expect("Error occured when writing to file");
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
