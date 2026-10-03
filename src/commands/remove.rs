use crate::store;

use anyhow::{Result, bail};

pub fn remove(theme_names: Vec<String>) -> Result<()> {
    let mut saved = store::load()?;

    for theme_name in theme_names {
        if saved.themes.remove(&theme_name).is_none() {
            bail!("Theme '{theme_name}' not found");
        }
    }

    store::save(&saved)
}
