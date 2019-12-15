#![feature(slice_patterns)]
use std::env;
use std::fs;

fn main() {
    let string_args = env::args().collect::<Vec<String>>();
    let str_args = string_args
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();

    match &str_args[..] {
        [_, query, filename] => do_stuff(query, filename),
        [exe, ..] => println!("Usage: {} pattern filename", exe),
        _ => println!("Invalid arguments"),
    }
}

fn do_stuff(query: &str, filename: &str) {
    println!("Searching for {} in {}", query, filename);

    let contents = fs::read_to_string(filename).expect("Failed to read the file");
    println!("File contents:\n{}", contents);
}
