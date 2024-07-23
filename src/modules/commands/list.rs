use crate::modules::file_handling;

pub fn list() {
    let config_file = file_handling::get_config_file();

    let themes = file_handling::read_config_file(&config_file);
    let current_theme = file_handling::get_current_theme();

    let mut theme_keys: Vec<&String> = themes.keys().map(|k| k).collect();
    theme_keys.sort();

    println!("---------- Available Themes ----------");
    for theme_name in theme_keys {
        let theme_image = themes.get(theme_name).unwrap();
        if theme_image == &current_theme {
            println!("* {:}", theme_name);
            continue;
        }
        println!("{:}", theme_name);
    }
}
