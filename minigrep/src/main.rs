#![feature(slice_patterns)]
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match parse_config(&args) {
        Ok(config) => do_stuff(&config),
        Err(err) => println!("{}", err),
    }
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

fn parse_config(args: &[String]) -> Result<Config, String> {
    let str_args = args
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();

    match &str_args[..] {
        [_, query, filename] => Ok(Config { query, filename }),
        [exe, ..] => Err(format!("Usage: {} pattern filename", exe)),
        _ => Err(String::from("Invalid arguments")),
    }
}

fn do_stuff(config: &Config) {
    println!("Searching for {} in {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename).expect("Failed to read the file");
    println!("File contents:\n{}", contents);
}
