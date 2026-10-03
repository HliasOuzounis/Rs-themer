use crate::{change_theme::change_theme, store, tui};

use std::io::{self, IsTerminal};

use anyhow::{Context, Result, bail};
use rand::{rng, seq::IteratorRandom};

pub fn select(theme_name: Option<String>, random: bool) -> Result<()> {
    let themes = store::load()?.themes;

    if themes.is_empty() {
        bail!("No themes saved. Add one with `wlr add <image>`");
    }

    let path = if random {
        themes.values().choose(&mut rng()).unwrap().clone()
    } else if let Some(name) = theme_name {
        themes
            .get(&name)
            .with_context(|| format!("Theme '{name}' not found"))?
            .clone()
    } else {
        if !io::stdout().is_terminal() {
            bail!("No theme name provided. Use --random or specify a name");
        }
        let current = store::get_current_theme();
        match tui::pick(&themes, current.as_deref())? {
            Some(path) => path,
            None => return Ok(()),
        }
    };

    change_theme(&path)
}
