use crate::database::Db;
use anyhow::bail;
use regex::Regex;
use std::cell::RefMut;

#[derive(Debug, PartialEq, Clone)]
pub enum Directive {
    DaisyHotSwap(String),
}

pub fn process_directive(input: &str, mut db: RefMut<Db>) {
    match parse_directive(input) {
        Ok(Directive::DaisyHotSwap(filename)) => match Db::load_from_file(&filename) {
            Ok(new_db) => {
                *db = new_db;
                println!(
                    "daisy wheel data has been loaded successfully from {}",
                    filename
                );
            }
            Err(err) => println!("loading daisy wheel data failed: {}", err),
        },
        Err(e) => println!("directive processing failed: {}", e),
    }
}

pub fn parse_directive(input: &str) -> anyhow::Result<Directive> {
    let re = Regex::new(r"@>(?P<dir>\w+)\s+(?P<arg>\S+)")?;
    if let Some(cap) = re.captures_iter(input).next() {
        let dir = cap["dir"].to_string();
        let arg = cap["arg"].to_string();
        if dir == *"daisy" {
            return Ok(Directive::DaisyHotSwap(arg));
        }
    }
    bail!("directive is not known")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_daisy_hot_swap_directive() {
        let res = parse_directive("@>daisy wheels/German.toml");
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Directive::DaisyHotSwap(String::from("wheels/German.toml"))
        );
    }
}
