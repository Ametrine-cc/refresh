use std;
use std::process::Command;

pub fn init(path: &str) {
    let mut cmd = Command::new("git");
    cmd.arg("init");
    cmd.arg(path);
    cmd.status().expect("failed to initialize git repository");
    println!("Git repository initialized successfully!");
}
