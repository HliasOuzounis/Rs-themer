use std::io::{self, Write};

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{Shell, generate};

use crate::cli::Cli;

// Theme names live in themes.toml, so the generated script can't list them statically.
// This helper asks wlr for them each time completion runs.
const ZSH_THEMES_HELPER: &str = include_str!("completions/themes.zsh");

pub fn print(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    let mut buf = Vec::new();
    generate(shell, &mut cmd, bin_name, &mut buf);
    let mut script = String::from_utf8(buf)?;
    if shell == Shell::Zsh {
        script = add_zsh_theme_completion(&script);
    }
    io::stdout().write_all(script.as_bytes())?;
    Ok(())
}

fn add_zsh_theme_completion(script: &str) -> String {
    // `#compdef wlr` must stay on the first line or compinit ignores the file
    let (compdef, rest) = script.split_once('\n').unwrap_or((script, ""));
    format!("{compdef}\n{ZSH_THEMES_HELPER}{rest}")
        .replace("Theme name:_default", "Theme name:_wlr_themes")
}
