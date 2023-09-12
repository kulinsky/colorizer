use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;
use clap::Parser;
use yaml_rust::YamlLoader;

pub mod color;
pub mod processor;

const APP_NAME: &str = "colorizer";

/// Simple program to highlight words in terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// color to use for highlighting
    #[arg(short, long, default_value = "red")]
    color: color::Color,

    /// regex pattern to find and colorize
    #[arg(short, long, num_args(0..))]
    regex: Vec<regex::Regex>,

    /// profile to use from config file
    #[arg(short, long, default_value = "default")]
    profile: String,
}

fn main() {
    let args = Args::parse();

    let patterns = get_patterns_from_cfg_by_profile(&args.profile)
        .unwrap_or(vec![])
        .iter()
        .chain(
            args.regex
                .iter()
                .map(|r| (args.color.clone(), r.clone()))
                .collect::<Vec<(color::Color, regex::Regex)>>()
                .iter(),
        )
        .cloned()
        .collect::<Vec<(color::Color, regex::Regex)>>();

    let patterns = patterns
        .iter()
        .map(|(c, r)| (c.clone().into(), r.clone()))
        .collect();

    let processor = processor::TextProcessor::new(patterns);

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let res = processor.process_line(line.unwrap());
        println!("{}", res);
    }
}

fn get_patterns_from_cfg_by_profile(
    profile_name: &str,
) -> Result<Vec<(color::Color, regex::Regex)>, anyhow::Error> {
    let cfg_path = home::home_dir()
        .ok_or(anyhow!("failed to get home path"))?
        .join(".config")
        .join(APP_NAME)
        .join("config.yml");

    let cfg_as_str = std::fs::read_to_string(cfg_path)?;

    let cfg = YamlLoader::load_from_str(&cfg_as_str)?;

    let profiles = cfg
        .get(0)
        .and_then(|cfg_root| cfg_root["profiles"].as_hash())
        .ok_or(anyhow!("no profiles found"))?;

    let res: Vec<(color::Color, regex::Regex)> = profiles
        .iter()
        .filter_map(|(k, v)| {
            let k_str = k.as_str()?;
            if k_str == profile_name {
                Some(v)
            } else {
                None
            }
        })
        .flat_map(|v| {
            v.as_hash().map(|colors| {
                colors.iter().flat_map(|(k, v)| {
                    let k_str = k.as_str().unwrap();
                    let v_vec = v.as_vec().unwrap();
                    v_vec.iter().filter_map(|v| {
                        v.as_str().map(|v_str| {
                            (
                                k_str.to_string().into(),
                                regex::Regex::from_str(v_str).unwrap(),
                            )
                        })
                    })
                })
            })
        })
        .flatten()
        .collect();

    Ok(res)
}
