use crate::modules::file_handling;

pub fn list() {
    if !file_handling::config_file_exists() {
        panic!("Themes file doesn't exist yet. Try adding some themes first")
    }
    let themes = file_handling::load_file();
    let current_theme = file_handling::get_current_theme();

    println!("---------- Available Themes ----------");
    for (theme, image) in themes.iter() {
        if image == &current_theme {
            println!("* {:}", theme);
            continue;
        }
        println!("{:}", theme);
    }
}
