use crate::{change_theme::change_theme, store};

use anyhow::{Context, Result, bail};
use rand::{rng, seq::IteratorRandom};

pub fn select(theme_name: Option<String>, random: bool) -> Result<()> {
    let themes = store::load()?.themes;

    if themes.is_empty() {
        bail!("No themes saved. Add one with `wlr add <image>`");
    }

    let path = if random {
        themes.values().choose(&mut rng()).unwrap()
    } else {
        let name = theme_name.context("No theme name provided. Use --random or specify a name")?;
        themes
            .get(&name)
            .with_context(|| format!("Theme '{name}' not found"))?
    };

    change_theme(path)
}
