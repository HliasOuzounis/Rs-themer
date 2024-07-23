use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;


pub fn get_config_file() -> fs::File {
    let config_file = env::var("XDG_CONFIG_HOME").unwrap() + "/themes/themes";
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_file)
        .expect("Could not open themes file")
}

pub fn get_empty_config_file() -> fs::File {
    let config_file = env::var("XDG_CONFIG_HOME").unwrap() + "/themes/themes";
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(config_file)
        .expect("Could not open themes file")
}

pub fn read_config_file(config_file: &fs::File) -> HashMap<String, String> {
    let mut themes_dictionary = HashMap::<String, String>::new();
    let reader = io::BufReader::new(config_file);
    for line in reader.lines() {
        let theme = line.unwrap();
        let theme_vec: Vec<&str> = theme.split(":").collect();
        themes_dictionary.insert(theme_vec[0].to_string(), theme_vec[1].to_string());
    }
    return themes_dictionary;
}

pub fn get_current_theme() -> String {
    let file_path = env::var("XDG_CACHE_HOME").unwrap() + "/wal/wal";
    let file_content = fs::read_to_string(file_path).expect("Could not read current theme file");
    return file_content;
}
