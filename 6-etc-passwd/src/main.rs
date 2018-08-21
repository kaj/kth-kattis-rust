use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn cat(filename: &str) -> Result<()> {
    let reader = BufReader::new(try!(File::open(filename)));
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn main() {
    if let Err(e) = cat("/etc/passwd") {
        panic!("Error: {}", e);
    }
}
