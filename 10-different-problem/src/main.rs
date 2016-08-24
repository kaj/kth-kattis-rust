//! Computes the difference between non-negative integers.
//! Each line of the input consists of a pair of integers.
//! Each integer is between 0 and 1015 (inclusive).
//! The input is terminated by end of file.
use std::io;

/// Read a line from standard input and parse it to a vec of numbers.
/// Panics if there are non-numbers in the line.
fn read_numbers() -> Option<Vec<i64>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok()
        .map(|_| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
}

fn main() {
    while let Some(v) = read_numbers() {
        if v.len() != 2 {
            break;
        }
        println!("{}", (v[0] - v[1]).abs());
    }
}
