// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright (c) 2025 Ametrine Foundation <business@ametrine.cc>

use std::io::{self, Read, Write};
use std::process::Command;
use std::process::exit;

use crate::gitty;
use crate::restock;

fn make_project(lang: &str, path: &str) {
    let mut cmd = Command::new("clear");
    cmd.status().expect("failed to clear terminal");

    match lang {
        "Rust" => {
            println!("Initializing Git repository...");
            gitty::init(path);
            restock::mk_project(path, lang);
        }
        "Python" => {
            println!("Creating Git repository...");
            gitty::init(path);
            restock::mk_project(path, lang);
        }
        "C#" => {
            println!("Creating Git repository...");
            gitty::init(path);
            restock::mk_project(path, lang);
        }
        "JavaScript" => {
            println!("Creating Git repository...");
            gitty::init(path);
            restock::mk_project(path, lang);
        }
        "Go" => {
            println!("Creating Git repository...");
            gitty::init(path);
            restock::mk_project(path, lang);
        }
        "Java" => {
            println!("Creating Git repository...");
            gitty::init(path);
            restock::mk_project(path, lang);
        }
        _ => todo!("Implement support for other languages"),
    }
}

pub fn choose_lang(new_path: &str) {
    let options = vec!["Rust", "Python", "Go", "Java", "C#", "JavaScript", "Exit"];

    match show_menu(&options) {
        Some(selection) => {
            // options[selection] => an option

            if options[selection] == "Exit" {
                let mut cmd = Command::new("clear");
                cmd.status().expect("failed to end program");
                exit(0);
            } else {
                match options[selection] {
                    "Rust" => {
                        make_project("Rust", new_path);
                    }
                    "Python" => {
                        make_project("Python", new_path);
                    }
                    "C#" => {
                        make_project("C#", new_path);
                    }
                    "Go" => {
                        make_project("Go", new_path);
                    }
                    "JavaScript" => {
                        make_project("JavaScript", new_path);
                    }
                    "Java" => {
                        make_project("Java", new_path);
                    }
                    _ => todo!("Implement support for other languages"),
                }
            }
        }
        None => {
            println!("\n✗ Selection cancelled");
        }
    }
}

fn show_menu(options: &[&str]) -> Option<usize> {
    let mut selected = 0;

    // Enable raw mode for reading key presses
    enable_raw_mode();

    loop {
        // Clear screen and move cursor to top
        print!("\x1B[2J\x1B[H");

        println!("Refresh  |  Ametrine Foundation");

        println!("Select a language to for your project");
        println!("Use ↑/↓ arrows to navigate, Enter to select, Esc to cancel\n");

        // Display options
        for (i, option) in options.iter().enumerate() {
            if i == selected {
                println!("→ \x1B[7m{}\x1B[0m", option); // Inverted colors for selected
            } else {
                println!("  {}", option);
            }
        }

        io::stdout().flush().unwrap();

        // Read a single byte
        let mut buf = [0u8; 3];
        io::stdin().read_exact(&mut buf[..1]).ok()?;

        match buf[0] {
            // Enter key
            b'\n' | b'\r' => {
                disable_raw_mode();
                return Some(selected);
            }
            // Escape key
            27 => {
                // Check if it's an arrow key (escape sequence)
                if io::stdin().read_exact(&mut buf[1..3]).is_ok() {
                    if buf[1] == b'[' {
                        match buf[2] {
                            b'A' => {
                                // Up arrow
                                if selected > 0 {
                                    selected -= 1;
                                }
                            }
                            b'B' => {
                                // Down arrow
                                if selected < options.len() - 1 {
                                    selected += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                } else {
                    // Just escape key, cancel
                    disable_raw_mode();
                    return None;
                }
            }
            // Vim-style navigation
            b'k' | b'K' => {
                if selected > 0 {
                    selected -= 1;
                }
            }
            b'j' | b'J' => {
                if selected < options.len() - 1 {
                    selected += 1;
                }
            }
            b'q' | b'Q' => {
                disable_raw_mode();
                return None;
            }
            _ => {}
        }
    }
}

#[cfg(unix)]
fn enable_raw_mode() {
    use std::os::unix::io::AsRawFd;
    let fd = io::stdin().as_raw_fd();
    let mut termios = std::mem::MaybeUninit::uninit();
    unsafe {
        libc::tcgetattr(fd, termios.as_mut_ptr());
        let mut termios = termios.assume_init();
        termios.c_lflag &= !(libc::ICANON | libc::ECHO);
        libc::tcsetattr(fd, libc::TCSANOW, &termios);
    }
}

#[cfg(unix)]
fn disable_raw_mode() {
    use std::os::unix::io::AsRawFd;
    let fd = io::stdin().as_raw_fd();
    let mut termios = std::mem::MaybeUninit::uninit();
    unsafe {
        libc::tcgetattr(fd, termios.as_mut_ptr());
        let mut termios = termios.assume_init();
        termios.c_lflag |= libc::ICANON | libc::ECHO;
        libc::tcsetattr(fd, libc::TCSANOW, &termios);
    }
}
