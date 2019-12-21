#![feature(slice_patterns)]
use std::error::Error;
use std::fs;

pub struct Config<'a> {
  query: &'a str,
  filename: &'a str,
}

impl Config<'_> {
  pub fn new(args: &[String]) -> Result<Config, String> {
    let str_args = args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

    match &str_args[..] {
      [_, query, filename] => Ok(Config { query, filename }),
      [exe, ..] => Err(format!("Usage: {} pattern filename", exe)),
      _ => Err(String::from("Invalid arguments")),
    }
  }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  let results = search(config.query, contents.as_str());
  println!("{}", results.join("\n"));
  Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn single_result() {
    let query = "pair";
    let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.";
    assert_eq!(
      search(query, contents),
      ["Then there's a pair of us - don't tell!"]
    );
  }
}
