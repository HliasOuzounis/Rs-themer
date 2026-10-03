use crate::store;

pub fn remove(theme_names: Vec<String>) {
    let mut saved = store::load();

    for theme_name in theme_names {
        saved
            .themes
            .remove(&theme_name)
            .expect("No such theme is saved");
    }

    store::save(&saved);
}
