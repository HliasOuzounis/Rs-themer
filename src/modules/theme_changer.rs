use std::process::Command;

pub fn change_theme(image_path: &str) {
    let _output = Command::new("wal")
        .args(&["-i", image_path, "--saturate", "0.5"])
        .output()
        .expect("Could not change theme");
}

pub fn reload_qtile() {
    let _output = Command::new("qtile")
        .args(&["cmd-obj", "-o", "cmd", "-f", "restart"])
        .output()
        .expect("Could not change theme");
}


pub fn change_alacritty() {
    let _output = Command::new("/home/hlias/.local/bin/alacritty_theme")
        .output()
        .expect("Could not change alacritty config");
}

pub fn reload_pywalfox() {
    let _output = Command::new("/home/hlias/.local/bin/pywalfox")
        .arg("update")
        .output()
        .expect("Could not change theme");
}
