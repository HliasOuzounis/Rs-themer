use crate::modules::{file_handling, theme_changer};
use rand::{seq::IteratorRandom, rng};

pub fn select(theme_name: Option<String>, random: bool) {
    let config_file = file_handling::get_config_file();
    let themes = file_handling::read_config_file(&config_file);

    if themes.is_empty() {
        eprintln!("Error: No themes found in config file.");
        std::process::exit(1);
    }

    if random {
        // Cleaner way to get a random key-value pair from a Map
        let mut rng_mod = rng();
        if let Some((_name, path)) = themes.iter().choose(&mut rng_mod) {
            theme_changer::change_theme(path);
        }
    } else {
        match theme_name {
            Some(name) => {
                if let Some(path) = themes.get(&name) {
                    theme_changer::change_theme(path);
                } else {
                    eprintln!("Error: Theme '{}' not found.", name);
                    std::process::exit(1);
                }
            }
            None => {
                eprintln!("Error: No theme name provided. Use --random or specify a name.");
                std::process::exit(1);
            }
        }
    }
}