//! Computes the difference between non-negative integers.
//! Each line of the input consists of a pair of integers.
//! Each integer is between 0 and 10^15 (inclusive).
//! The input is terminated by end of file.
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Some((a, b)) = line.ok().and_then(|line| line2numbers(&line)) {
            println!("{}", (a - b).abs());
        }
    }
}

fn line2numbers(line: &str) -> Option<(i64, i64)> {
    let mut iter = line.split_whitespace().filter_map(|s| s.parse().ok());
    iter.next().and_then(|a| iter.next().map(|b| (a, b)))
}
