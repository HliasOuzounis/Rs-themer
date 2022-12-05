use crate::modules::{file_handling, theme_changer};

use rand::{thread_rng, Rng};

pub fn select(
    theme_name: Option<String>,
    qtile: bool,
    pywalfox: bool,
    alacritty: bool,
    random: bool,
) {
    if !file_handling::config_file_exists() {
        panic!("Themes file doesn't exist yet. Try adding some themes first")
    }

    let themes = file_handling::load_file();
    if random {
        let random_num = thread_rng().gen_range(0..themes.len());
        let random_theme = themes.keys().skip(random_num).next().unwrap();
        theme_changer::change_theme(themes.get(random_theme).unwrap());
    } else if theme_name.is_none() {
        return;
    } else {
        theme_changer::change_theme(themes.get(&theme_name.unwrap()).unwrap());
    }
    if qtile {
        theme_changer::reload_qtile();
    }
    if pywalfox {
        theme_changer::reload_pywalfox();
    }
    if alacritty {
        theme_changer::change_alacritty();
    }
}
