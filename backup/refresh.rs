use std::env;
use std::io;

mod configuration;
mod restock;

fn main() {
    let args: Vec<String> = env::args().collect();

    fn args_is_empty() {
        println!("Usage: refresh <command>");
        println!("Use refresh --help for more information");
    }

    if args.len() < 2 {
        args_is_empty();
        return;
    }

    let command = &args[1];
    let path = &args[2];

    match command.as_str() {
        "init" => restock::init(path),
        "restock" => restock::restock(),
        _ => args_is_empty(),
    }

    configuration::load();

    println!("Refresh completed successfully!");
}
