use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_insensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Self, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();
    let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { query, filename, case_insensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_insensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut result = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\n Rust:\nsafe, fast, productive.\n Pick three.\n Duct tape.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents))
  }

  #[test]
  fn case_insensitive() {
    let query = "rUst";
    let contents = "Rust:\nsafe, fast, productive.\n Pick three.\nTrust me.";
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    )

  }
}
