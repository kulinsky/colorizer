use std::io;
use std::io::BufRead;

fn main() {
    let quote = "\x1b[0;32m@\x1b[0m".replace("@", r#"""#);
    
    for line in io::stdin().lock().lines() {
        let line = line.expect("Could not read line from standard in");

        let new_line = line.replace(r#"""#, &*quote);

        println!("{}", new_line);
    }
}
