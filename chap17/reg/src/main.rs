use std::io::BufRead;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SEMVER: Regex
    = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?")
    .expect("error parsing regex");

}

fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let Some(match_) = SEMVER.find(&line) {
            println!("{}", match_.as_str());
        }
    }
}
