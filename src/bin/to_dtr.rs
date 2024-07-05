extern crate rust_to_dtr;

use rust_to_dtr::parse_to_dtr;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get the input argument
    let input = &args[2];

    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");

    println!("{}", parse_to_dtr(&contents).unwrap_or_default());
}
