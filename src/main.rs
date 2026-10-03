mod modules;
use modules::commands;

use std::path::PathBuf;
use std::io::{self, Write};
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

#[derive(Debug, Args)]
struct ListOptions {
    /// Print only theme names, one per line
    #[arg(short, long)]
    plain: bool,
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
        WlrCommands::List(opt) => {
            commands::list::list(opt.plain);
        }
        WlrCommands::Completions { shell } => {
            let mut cmd = CLI::command();
            let bin_name = cmd.get_name().to_string();
            let mut buf = Vec::new();
            generate(shell, &mut cmd, bin_name, &mut buf);
            let mut script = String::from_utf8(buf).expect("Completion script is not valid UTF-8");
            if shell == Shell::Zsh {
                script = add_zsh_theme_completion(&script);
            }
            io::stdout()
                .write_all(script.as_bytes())
                .expect("Error writing completions");
        }
    }
}

// Theme names live in themes.toml, so the generated script can't list them statically.
// This helper asks wlr for them each time completion runs.
const ZSH_THEMES_HELPER: &str = r#"_wlr_themes() {
  # Split on newlines only, so names with spaces stay whole
  local -a themes
  themes=(${(f)"$(wlr list --plain 2>/dev/null)"})

  compadd -a themes
}
"#;

fn add_zsh_theme_completion(script: &str) -> String {
    // `#compdef wlr` must stay on the first line or compinit ignores the file
    let (compdef, rest) = script.split_once('\n').unwrap_or((script, ""));
    format!("{compdef}\n{ZSH_THEMES_HELPER}{rest}")
        .replace("Theme name:_default", "Theme name:_wlr_themes")
}
