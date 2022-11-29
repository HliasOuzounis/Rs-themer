use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "theme-picker", about = "Easy way to switch between multiple pywal themes")]
enum ThemePicker {
    /// <image-path> add new theme to selections
    Add {
        /// image path
        #[structopt(parse(from_os_str))]
        image_path: PathBuf,

        /// theme name (will default to image_name)
        theme_name: Option<String>,
    },
    /// change theme to selection8uj
    Select {
        /// theme name
        theme_name: Option<String>,
    },
    /// [theme name] remove theme from selections
    Remove {
        /// theme name
        theme_name: Option<String>,
    },
}

fn main() {
    let opt = ThemePicker::from_args();
    println!("{:?}", opt);
}