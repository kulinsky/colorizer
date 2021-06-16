use std::io;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, Read};

use clap::Clap;
use serde::Deserialize;
use serde_json::{Map, Value};
use ansi_term::Colour::{Blue, Cyan, Yellow, Red, Green, Purple};

/// Simple program to colorize any word
#[derive(Clap, Debug)]
#[clap(name = "colorizer", version = "0.1.0")]
struct CliConfig {
    /// Path to config.json file
    #[clap(short, long,  default_value = "~/.colorizer/settings.json")]
    config: String,

    /// Profile from config to use
    #[clap(short, long, default_value = "default")]
    profile: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    substrings: HashMap<String, String>,
}

impl Config {
    fn automatic() -> Result<Config, &'static str> {
        let args = CliConfig::parse();

        let mut file = File::open(args.config).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        let parsed: Value = serde_json::from_str(&buff).unwrap();
        let obj: &Map<String, Value> = parsed.as_object().unwrap();

        if obj.contains_key(&*args.profile) {
            let val = obj.get(&*args.profile).unwrap().clone();
            return Ok(serde_json::from_value(val).unwrap())
        }

        return Err("profile not found");
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
