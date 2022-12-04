use std::process::Command;

pub fn change_theme(image_path: &str) {
    Command::new("wal")
        .args(&["-i", image_path, "--saturate", "0.5"])
        .output()
        .expect("Could not change theme");
}

pub fn reload_qtile() {
    Command::new("qtile")
        .args(&["cmd-obj", "-o", "cmd", "-f", "restart"])
        .output()
        .expect("Could not change theme");
}

pub fn reload_pywalfox() {
    Command::new("pywalfox")
        .arg("update")
        .output()
        .expect("Could not change theme");
}
