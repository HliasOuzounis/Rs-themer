use std::path::PathBuf;
use structopt::StructOpt;

mod modules;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "theme-picker",
    author = "HliasOuzounis",
    about = "Easy way to switch between multiple pywal themes"
)]
struct CLI {
    #[structopt(subcommand)]
    cmd: ThemePicker,
}

#[derive(Debug, StructOpt)]
enum ThemePicker {
    #[structopt(name = "add", about = "<image-path> add new theme to selections")]
    Add(AddOptions),
    #[structopt(name = "remove", about = "[theme name] remove theme from selections")]
    Remove(RemoveOptions),
    #[structopt(name = "select", about = "change theme to selection")]
    Select(SelectOptions),
}
#[derive(Debug, StructOpt)]
pub struct AddOptions {
    /// image path
    #[structopt(parse(from_os_str))]
    image_path: PathBuf,

    /// theme name (will default to image_name)
    theme_name: Option<String>,
}
#[derive(Debug, StructOpt)]
struct RemoveOptions {
    /// theme name
    theme_name: Option<String>,
}
#[derive(Debug, StructOpt)]
struct SelectOptions {
    /// theme name
    theme_name: Option<String>,
}

fn main() {
    let args = CLI::from_args();
    match args.cmd {
        ThemePicker::Add(opt) => {
            modules::add::add(opt.image_path, opt.theme_name);
        }
        ThemePicker::Remove(opt) => {
            modules::remove::remove(opt.theme_name);
        }
        ThemePicker::Select(opt) => {
            modules::select::select(opt.theme_name);
        }
    }
}
