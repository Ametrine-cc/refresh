use std::fs;
use std::process::Command;

pub fn mk_project(path: &str, lang: &str) {
    match lang {
        "Rust" => {
            println!("Creating Rust project...");
            let mut cmd = Command::new("cargo");
            cmd.arg("init");
            cmd.arg(path);
            cmd.status().expect("failed to create project");
            println!("Rust project created successfully!");
        }

        "Python" => {
            println!("Creating Python project...");
            let mut cmd = Command::new("python");
            cmd.arg("-m");
            cmd.arg("venv");
            cmd.arg(path);
            cmd.status().expect("failed to create project");
            println!("Python project created successfully!");
        }
        _ => unreachable!(),
    }
}
