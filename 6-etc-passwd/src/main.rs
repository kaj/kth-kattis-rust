use std::io::{BufReader, BufRead, Result};
use std::fs::File;

fn cat(filename: &str) -> Result<()> {
    let reader = BufReader::new(try!(File::open(filename)));
    for line in reader.lines() {
        println!("{}", try!(line));
    };
    Ok(())
}

fn main() {
    if let Err(e) = cat("/etc/passwd") {
        panic!("Error: {}", e);
    }
}
