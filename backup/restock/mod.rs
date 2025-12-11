use std::fs;
use std::io;
use std::process::Command;

pub fn init(path: &str) {
    let mut cmd = Command::new("git");
    cmd.arg("init");
    cmd.status().expect("failed to execute git init");
}

pub fn restock() {
    let mut cmd = Command::new("git");
    cmd.arg("pull");
    cmd.status().expect("failed to execute git pull");
}
