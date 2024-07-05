extern crate rust_to_dtr;

use rust_to_dtr::parse_to_dtr;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if we have the correct number of arguments
    if args.len() != 2 {
        args.iter().for_each(|arg| println!("{}", arg));

        eprintln!("Usage: <filepath>");
        std::process::exit(1);
    }

    // Get the input argument
    let input = &args[1];

    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");

    println!("{}", parse_to_dtr(&contents).unwrap_or_default());
}
