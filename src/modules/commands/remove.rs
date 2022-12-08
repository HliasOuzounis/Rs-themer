use std::io::Write;

use crate::modules::file_handling;

pub fn remove(theme_name: Option<String>) {
    if !file_handling::config_file_exists() {
        panic!("Themes file doesn't exist yet. Try adding some themes first")
    }
    if theme_name.is_none() {
    } else {
        let theme_name = theme_name.unwrap();
        let mut themes = file_handling::load_file();

        themes.remove(&theme_name).expect("No such theme is saved");

        let mut config_file =
            file_handling::create_config_file().expect("Could not remove from file");

        for theme in themes {
            config_file
                .write_fmt(format_args!("{}:{}\n", theme.0, theme.1))
                .expect("Error occured when writing to file");
        }
    }
}
