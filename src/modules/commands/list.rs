use crate::modules::file_handling;

pub fn list() {
    if !file_handling::config_file_exists() {
        panic!("Themes file doesn't exist yet. Try adding some themes first")
    }

    let themes = file_handling::load_file();
    println!("---------- Available Themes ----------");
    for theme in themes.keys() {
        println!("{:}", theme);
    }
}
