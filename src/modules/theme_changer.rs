use std::process::Command;

pub fn change_theme(image_path: &str) {
    let _output = Command::new("wallust")
        .args(&["run", image_path])
        .output()
        .expect("Could not change theme");
}