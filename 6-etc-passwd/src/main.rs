use std::io::{BufReader, BufRead, Read, Result};
use std::fs::File;

fn cat(filename: &str) -> Result<()> {
    let reader = BufReader::new(try!(File::open(filename)));
    for line in reader.lines() {
        println!("{}", try!(line));
    };
    Ok(())
}

fn main() {
    if let Err(e) = cat("/etc/passwdx") {
        panic!("Error: {}", e);
    }
}
