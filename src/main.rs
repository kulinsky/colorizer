use std::io;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, Read, Write};

use clap::Clap;
use serde::Deserialize;
use serde_json::{Map, Value};
use anyhow::{anyhow, Context, Result};
use ansi_term::Colour::{Blue, Cyan, Yellow, Red, Green, Purple};

/// Simple program to colorize any word
#[derive(Clap, Debug)]
#[clap(name = "colorizer", version = "0.1.0")]
struct CliConfig {
    /// Path to config.json file
    #[clap(short, long)]
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
    fn automatic() -> Result<Config> {
        let args = CliConfig::parse();

        let mut file = File::open(&*args.config)
            .with_context(|| format!("Failed to open config from {}", &*args.config))?;
        let mut buff = String::new();
        file.read_to_string(&mut buff)
            .with_context(|| format!("Failed to read config file {}", &*args.config))?;

        let parsed: Value = serde_json::from_str(&buff)
            .with_context(|| format!("Failed to parse json {}", args.config))?;
        let obj: &Map<String, Value> = parsed.as_object()
            .with_context(|| format!("Failed to parse json to object {}", &*args.config))?;

        let val = obj.get(&*args.profile)
            .with_context(|| format!("profile not found `{}`", &*args.profile))?.clone();

        let config = serde_json::from_value(val)?;

        Ok(config)
    }
}

fn colorize(color: &str, word: &str) -> Result<String> {
    match color {
        "RED" => Ok(Red.paint(word).to_string()),
        "GREEN" => Ok(Green.paint(word).to_string()),
        "BLUE" => Ok(Blue.paint(word).to_string()),
        "CYAN" => Ok(Cyan.paint(word).to_string()),
        "YELLOW" => Ok(Yellow.paint(word).to_string()),
        "PURPLE" => Ok(Purple.paint(word).to_string()),
        _ => Err(anyhow!("unknown color: {}", color))
    }
}

fn main() -> Result<()> {
    let conf = Config::automatic()?;
    let stdout = io::stdout();
    let mut buff = io::BufWriter::new(stdout.lock());

    for line in io::stdin().lock().lines() {
        let mut line = line.expect("Could not read line from standard in");

        for k in conf.substrings.keys() {
            line = line.replace(k, &*colorize(&*conf.substrings[k], k)?)
        }

        writeln!(&mut buff, "{}", line).ok();
    };

    Ok(())
}
