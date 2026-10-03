use crate::{change_theme::change_theme, store};
use rand::{rng, seq::IteratorRandom};

pub fn select(theme_name: Option<String>, random: bool) {
    let themes = store::load().themes;

    if themes.is_empty() {
        eprintln!("Error: No themes saved. Add one with `wlr add <image>`.");
        std::process::exit(1);
    }

    if random {
        // Cleaner way to get a random key-value pair from a Map
        let mut rng_mod = rng();
        if let Some((_name, path)) = themes.iter().choose(&mut rng_mod) {
            change_theme(path);
        }
    } else {
        match theme_name {
            Some(name) => {
                if let Some(path) = themes.get(&name) {
                    change_theme(path);
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
