use std::fs;
use std::io::Write;
use std::path::PathBuf;

use super::file_handling;

pub fn add(image_path: PathBuf, user_name: Option<String>) {
    if !image_path.exists() {
        panic!("No such image exists");
    }
    let extension = image_path.extension().unwrap().to_str().unwrap();

    if !["jpg", "png"].contains(&extension) {
        panic!("Invalid file format. Please use jpg or png");
    }

    let full_path = fs::canonicalize(image_path).unwrap();

    let theme_name = if user_name.is_none() {
        String::from(full_path.file_stem().unwrap().to_str().unwrap())
    } else {
        user_name.unwrap()
    };

    let mut config_file: fs::File;
    if !file_handling::config_file_exists() {
        config_file = file_handling::create_config_file().unwrap();
    } else {
        config_file = file_handling::append_to_config_file();
    }
    let theme = format!("{}:{}\n", theme_name, full_path.to_str().unwrap());

    config_file
        .write(theme.as_bytes())
        .expect("Couldn't save theme to file");
}
