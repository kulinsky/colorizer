use std::io::{self, BufRead};

use clap::Parser;

pub mod colorizer;
pub mod finder;
pub mod processor;

const EMAIL_REGEX: &str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
const IPV4_REGEX: &str =
    r#"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)"#;
const ISO_TIME_REGEX: &str = r#"(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:Z|[+-][01]\d:[0-5]\d)"#;

/// Simple program highlight words in terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// color to use for highlighting
    #[arg(short, long, default_value = "red")]
    color: String,

    /// find and colorize emails
    #[arg(short, long, default_value_t = false)]
    email: bool,

    /// find and colorize ipv4 addresses
    #[arg(long, default_value_t = false)]
    ipv4: bool,

    /// find and colorize iso time strings
    #[arg(long, default_value_t = false)]
    iso_time: bool,

    /// regex pattern to find and colorize
    #[arg(short, long, num_args(0..))]
    pattern: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mut patterns = args.pattern;

    if args.email {
        patterns.push(EMAIL_REGEX.to_string());
    }

    if args.ipv4 {
        patterns.push(IPV4_REGEX.to_string());
    }

    if args.iso_time {
        patterns.push(ISO_TIME_REGEX.to_string());
    }

    let console_colorizer = colorizer::ConsoleColorizer::new(Some(args.color));

    let regex_finder = finder::RegexFinder::new(patterns);

    let processor = processor::TextProcessor::new(console_colorizer, regex_finder);

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let res = processor.process_line(line.unwrap());
        println!("{}", res);
    }
}
