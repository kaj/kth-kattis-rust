//! Computes the difference between non-negative integers.
//! Each line of the input consists of a pair of integers.
//! Each integer is between 0 and 10^15 (inclusive).
//! The input is terminated by end of file.
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock()
                     .lines()
                     .map(|line| line.ok().and_then(line2numbers))
                     .take_while(|l| l.is_some())
                     .map(|l| l.unwrap());

    for v in lines.take_while(|v| v.len() == 2) {
        println!("{}", (v[0] - v[1]).abs());
    }
}

fn line2numbers(line: String) -> Option<Vec<i64>> {
    line.split_whitespace().map(|s| s.parse().ok()).collect()
}
