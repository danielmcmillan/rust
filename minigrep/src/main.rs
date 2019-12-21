use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    if let Err(err) = minigrep::run(&config) {
        println!("Error: {}", err);
        process::exit(1);
    }
}
