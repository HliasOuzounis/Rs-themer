use crate::modules::file_handling;

pub fn remove(theme_names: Vec<String>) {
    let mut store = file_handling::load();

    for theme_name in theme_names{
        store.themes.remove(&theme_name).expect("No such theme is saved");
    }

    file_handling::save(&store);
}
