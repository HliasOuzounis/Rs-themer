use std::env;
use std::env::VarError;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn load_file() -> std::io::Result<()> {
    let file_path = env::var("XDG_CONFIG_HOME").unwrap() + "/themes/themes";
    let file_content = fs::read_to_string(file_path).expect("Could not read themes file");
    let file_themes: Vec<Vec<&str>> = file_content
        .lines()
        .map(|x| x.split(':').collect())
        .collect();

    Ok(())
}

fn save_file(themes: Vec<(String, PathBuf)>) {}

pub fn check_file_exists() -> std::io::Result<()> {
    let config_home = env::var("XDG_CONFIG_HOME").unwrap() + "/themes";
    let directory = Path::new(&config_home);
    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }
    let file_path = config_home + "/themes";
    if !Path::new(&file_path).exists() {
        let file = fs::File::create(file_path);
    }
    Ok(())
}

pub fn config_file_exists() -> bool {
    let config_file = env::var("XDG_CONFIG_HOME").unwrap() + "/themes/themes";
    Path::new(&config_file).exists()
}

pub fn append_to_config_file() -> fs::File {
    let config_file = env::var("XDG_CONFIG_HOME").unwrap() + "/themes/themes";
    fs::OpenOptions::new()
        .append(true)
        .open(config_file)
        .unwrap()
}

pub fn create_config_file() -> std::io::Result<fs::File> {
    let config_home = env::var("XDG_CONFIG_HOME").unwrap() + "/themes";
    let directory = Path::new(&config_home);
    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }
    let file_path = config_home + "/themes";
    fs::File::create(file_path)
}
