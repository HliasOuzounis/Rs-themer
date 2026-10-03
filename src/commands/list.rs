use crate::store;

pub fn list(plain: bool) {
    let themes = store::load().themes;

    // One name per line, for scripts and shell completion
    if plain {
        for theme_name in themes.keys() {
            println!("{}", theme_name);
        }
        return;
    }

    let current_theme = store::get_current_theme();

    println!("---------- Available Themes ----------");
    for (theme_name, theme_image) in &themes {
        if current_theme.as_ref() == Some(theme_image) {
            println!("* {}", theme_name);
            continue;
        }
        println!("{}", theme_name);
    }
}
