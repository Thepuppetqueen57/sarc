use std::fs::read_to_string;
use std::process::exit;

mod parser;
use parser::*;

fn main() {
    let pos_args: Vec<String> = std::env::args().skip(1).collect();

    if pos_args.is_empty() {
        println!("No file specified!");
        exit(1)
    }

    let code = read_to_string(&pos_args[0])
        .expect("Failed to read file. Does it exist?");

    parse_code(code);
}