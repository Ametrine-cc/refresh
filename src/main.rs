use std::env;
use std::io;
mod configuration;
mod gitty;
mod restock;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "init" => {
            if args.len() < 3 {
                eprintln!("Error: init command requires a path");
                eprintln!("Usage: refresh init <path>");
                return;
            }
            let path = &args[2];
            configuration::choose_lang(path);
            println!("Refresh completed successfully!");
        }
        "--help" | "-h" => {
            print_usage();
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage: refresh <command> [arguments]");
    println!("\nCommands:");
    println!("  init <path>    Initialize a new project at the given path");
    println!("  --help, -h     Show this help message");
}
