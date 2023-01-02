use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub fn load_file() -> HashMap<String, String> {
    let file_path = env::var("HOME").unwrap() + "/.config/themes/themes";
    let file_content = fs::read_to_string(file_path).expect("Could not read themes file");

    let mut themes_dictionary = HashMap::<String, String>::new();

    for theme in file_content.lines().into_iter() {
        let theme_vec: Vec<&str> = theme.split(":").collect();
        themes_dictionary.insert(theme_vec[0].to_string(), theme_vec[1].to_string());
    }

    return themes_dictionary;
}

pub fn config_file_exists() -> bool {
    let config_file = env::var("HOME").unwrap() + "/.config/themes/themes";
    Path::new(&config_file).exists()
}


pub fn create_config_file() -> std::io::Result<fs::File> {
    let config_home = env::var("HOME").unwrap() + "/.config/themes";
    let directory = Path::new(&config_home);
    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }
    let file_path = [config_home, "themes".to_string()].join("/");
    fs::File::create(file_path)
}
