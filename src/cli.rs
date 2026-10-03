use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueHint};
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[command(
    name = "wlr",
    about = "A rust-based theme and wallpaper manager",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add new theme
    Add(AddOptions),
    /// Remove theme
    Remove(RemoveOptions),
    /// Change theme (opens a picker with previews when no name is given)
    Select(SelectOptions),
    /// List available themes
    List(ListOptions),

    #[command(hide = true)]
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Args)]
pub struct AddOptions {
    /// Image path(s)
    #[arg(value_hint = ValueHint::FilePath)] // Tells ZSH to suggest files
    pub image_path: Vec<PathBuf>,

    /// Set theme immediately
    #[arg(short, long)]
    pub set_theme: bool,
}

#[derive(Debug, Args)]
pub struct RemoveOptions {
    /// Theme name
    #[arg(value_name = "THEME_NAME")]
    pub theme_name: Vec<String>,
}

#[derive(Debug, Args)]
pub struct SelectOptions {
    /// Theme name
    #[arg(value_name = "THEME_NAME")]
    pub theme_name: Option<String>,

    /// Pick a random theme
    #[arg(short, long)]
    pub random: bool,
}

#[derive(Debug, Args)]
pub struct ListOptions {
    /// Print only theme names, one per line
    #[arg(short, long)]
    pub plain: bool,
}
