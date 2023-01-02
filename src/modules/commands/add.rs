use crate::modules::file_handling;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn add(image_path: PathBuf, user_name: Option<String>) {
    check_image_path(&image_path);

    let full_path = fs::canonicalize(image_path).unwrap();

    let theme_name = if user_name.is_none() {
        String::from(full_path.file_stem().unwrap().to_str().unwrap())
    } else {
        user_name.unwrap()
    };
    
    let mut existing_themes = file_handling::load_file();

    if existing_themes.contains_key(&theme_name){
        panic!("Theme with that name already exists")
    }
    existing_themes.insert(theme_name, full_path.to_str().unwrap().to_string());

    let mut config_file = file_handling::create_config_file().unwrap();

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
