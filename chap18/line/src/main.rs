use std::io;
use std::io::prelude::*;

fn grep<R>(target: &str, reader: R) -> io::Result<()>
    where R: BufRead
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() {
    let s = "tt";
    let stdin = io::stdin();
    println!("{:?}", grep(&s, stdin.lock()));
}
