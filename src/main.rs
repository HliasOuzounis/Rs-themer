mod change_theme;
mod cli;
mod commands;
mod completions;
mod store;
mod tui;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Add(opt) => commands::add::add(opt.image_path, opt.set_theme),
        Commands::Remove(opt) => commands::remove::remove(opt.theme_name),
        Commands::Select(opt) => commands::select::select(opt.theme_name, opt.random),
        Commands::List(opt) => commands::list::list(opt.plain),
        Commands::Completions { shell } => completions::print(shell),
    }
}
