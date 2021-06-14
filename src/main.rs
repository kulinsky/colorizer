use std::{io, env};
use std::io::{BufRead, Read};

use serde::Deserialize;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Config {
    substrings: HashMap<String, String>,
}

impl Config {
    fn automatic() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        let mut file = File::open(filename).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        Ok(serde_json::from_str(&buff).unwrap())
    }
}

fn get_symbols(color: &str) -> Result<(&str, &str), &'static str> {
    match color {
        "GREEN" => Ok(("\x1b[0;32m", "\x1b[0m")),
        "RED" => Ok(("\x1b[0;31m", "\x1b[0m")),
        "BLUE" => Ok(("\x1b[0;34m", "\x1b[0m")),
        _ => Err("unknown color ")
    }
}

fn main() {
    let conf = Config::automatic().unwrap();

    for line in io::stdin().lock().lines() {
        let line = line.expect("Could not read line from standard in");

        if conf.substrings.len() > 0 {
            let mut new_line = line;

            for k in conf.substrings.keys() {
                let s = get_symbols(&*conf.substrings[k]).unwrap();
                new_line = new_line.replace(k, &*format!("{}{}{}", s.0, k, s.1));
            }

            println!("{}", new_line);
        } else {
            println!("{}", line);
        }
    }
}
