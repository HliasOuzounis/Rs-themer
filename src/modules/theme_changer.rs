use std::path::Path;
use std::process::Command;

pub fn change_theme(image_path: &Path) {
    let _output = Command::new("wallust")
        .arg("run")
        .arg(image_path)
        .output()
        .expect("Could not change theme");
}
