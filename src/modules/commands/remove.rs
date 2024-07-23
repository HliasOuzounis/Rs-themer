use std::io::Write;

use crate::modules::file_handling;

pub fn remove(theme_names: Vec<String>) {
    let config_file = file_handling::get_config_file();
    let mut themes = file_handling::read_config_file(&config_file);

    for theme_name in theme_names{
        themes.remove(&theme_name).expect("No such theme is saved");
    }

    let mut config_file = file_handling::get_empty_config_file();
    for theme in themes {
        config_file
            .write_fmt(format_args!("{}:{}\n", theme.0, theme.1))
            .expect("Error occured when writing to file");
    }
}
