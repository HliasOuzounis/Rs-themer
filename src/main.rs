mod modules;
use modules::commands;

use std::path::PathBuf;
use std::io;
// Changed from structopt to clap
use clap::{Parser, Subcommand, Args, CommandFactory, ValueHint};
use clap_complete::{generate, Shell};

#[derive(Debug, Parser)]
#[command(
    name = "wlr",
    about = "A rust-based theme and wallpaper manager",
    version = "1.0.0"
)]
struct CLI {
    #[command(subcommand)]
    cmd: WlrCommands,
}

#[derive(Debug, Subcommand)]
enum WlrCommands {
    /// Add new theme
    Add(AddOptions),
    /// Remove theme
    Remove(RemoveOptions),
    /// Change theme
    Select(SelectOptions),
    /// List available themes
    List,

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
    image_path: Vec<PathBuf>,

    /// Set theme immediately
    #[arg(short, long)]
    set_theme: bool,
}

#[derive(Debug, Args)]
struct RemoveOptions {
    /// Theme name
    #[arg(value_name = "THEME_NAME")]
    theme_name: Vec<String>,
}

#[derive(Debug, Args)]
struct SelectOptions {
    /// Theme name
    #[arg(value_name = "THEME_NAME")]
    theme_name: Option<String>,

    #[arg(short, long)]
    random: bool,
}

fn main() {
    let args = CLI::parse();

    match args.cmd {
        WlrCommands::Add(opt) => {
            commands::add::add(opt.image_path, opt.set_theme);
        }
        WlrCommands::Remove(opt) => {
            commands::remove::remove(opt.theme_name);
        }
        WlrCommands::Select(opt) => {
            commands::select::select(opt.theme_name, opt.random);
        }
        WlrCommands::List => {
            commands::list::list();
        }
        WlrCommands::Completions { shell } => {
            let mut cmd = CLI::command();
            let bin_name = cmd.get_name().to_string();
            generate(shell, &mut cmd, bin_name, &mut io::stdout());
        }
    }
}