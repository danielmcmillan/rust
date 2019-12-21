#![feature(slice_patterns)]
use std::env;
use std::error::Error;
use std::fs;

pub struct Config<'a> {
  query: &'a str,
  filename: &'a str,
  case_sensitive: bool,
}

impl Config<'_> {
  pub fn new(args: &[String]) -> Result<Config, String> {
    let str_args = args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    match &str_args[..] {
      [_, query, filename, case_insensitive] => Ok(Config {
        query,
        filename,
        case_sensitive: case_insensitive != &"y",
      }),
      [_, query, filename] => Ok(Config {
        query,
        filename,
        case_sensitive,
      }),
      [exe, ..] => Err(format!("Usage: {} pattern filename", exe)),
      _ => Err(String::from("Invalid arguments")),
    }
  }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  let results = if config.case_sensitive {
    search(config.query, contents.as_str())
  } else {
    search_case_insensitive(config.query, contents.as_str())
  };
  println!("{}", results.join("\n"));
  Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "hello";
    let contents = "\
Hello world,
and hello to you";
    assert_eq!(search(query, contents), ["and hello to you"]);
  }

  #[test]
  fn case_insensitive() {
    let query = "hElLo";
    let contents = "\
Hello world,
and hello to you";
    assert_eq!(
      search_case_insensitive(query, contents),
      ["Hello world,", "and hello to you"]
    );
  }
}
