use std::{io, env};
use std::io::{BufRead, Read};

use serde::Deserialize;
use std::fs::File;
use std::collections::HashMap;
use ansi_term::Colour::{Blue, Cyan, Yellow, Red, Green, Purple};

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

fn colorize(color: &str, word: &str) -> Result<String, &'static str> {
    match color {
        "RED" => Ok(Red.paint(word).to_string()),
        "GREEN" => Ok(Green.paint(word).to_string()),
        "BLUE" => Ok(Blue.paint(word).to_string()),
        "CYAN" => Ok(Cyan.paint(word).to_string()),
        "YELLOW" => Ok(Yellow.paint(word).to_string()),
        "PURPLE" => Ok(Purple.paint(word).to_string()),
        _ => Err("unknown color ")
    }
}

fn main() {
    let conf = Config::automatic().unwrap();

    for line in io::stdin().lock().lines() {
        let mut line = line.expect("Could not read line from standard in");

        for k in conf.substrings.keys() {
            line = line.replace(k, &*colorize(&*conf.substrings[k], k).unwrap())
        }

        println!("{}", line);
    }
}
