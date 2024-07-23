use core::panic;

use crate::modules::{file_handling, theme_changer};

use rand::{thread_rng, Rng};

pub fn select(theme_name: Option<String>, qtile: bool, pywalfox: bool, random: bool) {
    let config_file = file_handling::get_config_file();
    let themes = file_handling::read_config_file(&config_file);
    
    if random {
        let random_num = thread_rng().gen_range(0..themes.len());
        let random_theme = themes.keys().skip(random_num).next().unwrap();
        theme_changer::change_theme(themes.get(random_theme).unwrap());
    } else if theme_name.is_none() {
        panic!("No theme name provided");
    } else {
        theme_changer::change_theme(themes.get(&theme_name.unwrap()).unwrap());
    }

    if pywalfox {
        theme_changer::reload_pywalfox();
    }
    if qtile {
        theme_changer::reload_qtile();
    }
}
