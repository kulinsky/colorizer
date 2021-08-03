use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

use ansi_term::Colour::{Black, Blue, Cyan, Green, Purple, Red, White, Yellow, Fixed};
use anyhow::{anyhow, Context, Result};
use clap::{App, AppSettings, Arg};
use regex::Regex;
use serde_json::{json, Value};

const DEFAULT_PROFILE: &str = "default";

const EMAIL_REGEX: &str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
const IPV4_REGEX: &str =
    r#"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)"#;
const ISO_TIME_REGEX: &str = r#"(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:Z|[+-][01]\d:[0-5]\d)"#;

fn colorize(color: &str, word: &str) -> Result<String> {
    match color {
        "BLACK" => Ok(Black.paint(word).to_string()),
        "RED" => Ok(Red.paint(word).to_string()),
        "GREEN" => Ok(Green.paint(word).to_string()),
        "BLUE" => Ok(Blue.paint(word).to_string()),
        "CYAN" => Ok(Cyan.paint(word).to_string()),
        "YELLOW" => Ok(Yellow.paint(word).to_string()),
        "PURPLE" => Ok(Purple.paint(word).to_string()),
        "WHITE" => Ok(White.paint(word).to_string()),
        "FORESTGREEN" => Ok(Fixed(22).paint(word).to_string()),
        "MAGENTA" => Ok(Fixed(200).paint(word).to_string()),
        "ORANGE" => Ok(Fixed(214).paint(word).to_string()),
        _ => Err(anyhow!("Unknown color: {}", color)),
    }
}

fn get_built_in() -> Value {
    json!({
        "nginx": {
            "regex": {
                "^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)": "ORANGE",
                "\\[([\\s\\S]+)\\]": "CYAN",
                "\"([A-Z]+) ([\\S]*) ([\\S]+)[\"]": "FORESTGREEN",
                " (\\d{3}) ": "MAGENTA",
                "[\"]([\\S]*)[\"] [\"]([\\S\\s]+)[\"]": "CYAN"
            }
        }
    })
}

fn parse_file(path: &str) -> Result<Value> {
    let mut buff = String::new();

    let mut file =
        File::open(&path).with_context(|| format!("Failed to open config from: {}", &path))?;

    file.read_to_string(&mut buff)
        .with_context(|| format!("Failed to read config file: {}", &path))?;

    let parsed: Value =
        serde_json::from_str(&buff).with_context(|| format!("Failed to parse json: {}", &path))?;

    Ok(parsed)
}

fn process_line(
    mut line: String,
    substrings: &[(&str, &str)],
    color_reg: &[(&str, Regex)],
) -> Result<()> {
    for (k, v) in substrings {
        line = line.replace(*k, &*colorize(v, k)?)
    }

    for (k, v) in color_reg {
        let l = String::from(&line);
        for cap in v.captures_iter(&*l) {
            line = line.replace(&cap[0], &*colorize(*k, &cap[0])?)
        }
    }

    println!("{}", line);
    Ok(())
}

fn main() -> Result<()> {
    let matches = App::new("Colorizer")
        .setting(AppSettings::ColoredHelp)
        .version("1.1.0")
        .about("Program to colorize any word.")
        .arg(Arg::new("INPUT")
            .about("Sets the input file to use")
            .required(false)
            .index(1))
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a custom config file")
            .takes_value(true)
            .required(false))
        .arg(Arg::new("profile")
            .short('p')
            .multiple_values(true)
            .multiple_occurrences(true)
            .takes_value(true)
            .about("Sets the profile to use, you can use multiple profiles (ex: colorizer -p profile1 profile2)"))

        .arg(Arg::new("ipv4")
            .value_name("COLOR")
            .long("ipv4")
            .takes_value(true)
            .required(false)
            .about("Shortcut for highlighting ipv4, takes the color as the value"))

        .arg(Arg::new("isotime")
            .value_name("COLOR")
            .takes_value(true)
            .long("isotime")
            .required(false)
            .about("Shortcut for highlighting time at iso format, takes the color as the value"))

        .arg(Arg::new("email")
            .value_name("COLOR")
            .takes_value(true)
            .long("email")
            .required(false)
            .about("Shortcut for highlighting email, takes the color as the value"))

        .get_matches();

    let mut substrings: Vec<(&str, &str)> = Vec::new();
    let mut color_reg: Vec<(&str, Regex)> = Vec::new();

    if let Some(value) = matches.value_of("ipv4") {
        color_reg.push((value, Regex::new(IPV4_REGEX).unwrap()))
    }
    if let Some(value) = matches.value_of("email") {
        color_reg.push((value, Regex::new(EMAIL_REGEX).unwrap()))
    }
    if let Some(value) = matches.value_of("isotime") {
        color_reg.push((value, Regex::new(ISO_TIME_REGEX).unwrap()))
    }

    let mut profiles: Vec<&str> = Vec::new();

    if let Some(values) = matches.values_of("profile") {
        for v in values {
            profiles.push(v);
        }
    }

    let parsed: Value;

    match matches.value_of("config") {
        None => {
            parsed = get_built_in();
        }
        Some(path) => {
            parsed = parse_file(path)?;
        }
    }

    if profiles.is_empty() {
        if matches.value_of("config").is_some() {
            let val = parsed
                .get(DEFAULT_PROFILE)
                .with_context(|| format!("Profile not found: {}", DEFAULT_PROFILE))?;

            if let Some(subs) = val.get("substrings") {
                for (k, v) in subs.as_object().unwrap() {
                    substrings.push((k, <&str>::clone(&v.as_str().unwrap())))
                }
            }
            if let Some(r) = val.get("regex") {
                for (k, v) in r.as_object().unwrap() {
                    color_reg.push((v.as_str().unwrap(), k.parse()?))
                }
            }
        }
    } else {
        for p in profiles {
            let val = parsed
                .get(p)
                .with_context(|| format!("Profile not found: {}", p))?;

            if let Some(r) = val.get("regex") {
                for (k, v) in r.as_object().unwrap() {
                    color_reg.push((v.as_str().unwrap(), k.parse()?))
                }
            }

            if let Some(r) = val.get("substrings") {
                for (k, v) in r.as_object().unwrap() {
                    substrings.push((k.as_str(), v.as_str().unwrap()))
                }
            }
        }
    }

    match matches.value_of("INPUT") {
        None => {
            for line in io::stdin().lock().lines() {
                let line = line.expect("Could not read line from standard in");
                process_line(line, &substrings, &color_reg)?;
            }
        }
        Some(filename) => {
            let file = File::open(filename)
                .with_context(|| format!("Failed to open target file: {}", filename))?;

            for line in io::BufReader::new(file).lines() {
                let line = line.expect("Could not read line from standard in");
                process_line(line, &substrings, &color_reg)?;
            }
        }
    };

    Ok(())
}
