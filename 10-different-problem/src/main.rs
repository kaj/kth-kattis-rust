use std::io;

fn read_numbers() -> Option<(i64, i64)> {
    let mut line = String::new();
    if let Ok(_) = io::stdin().read_line(&mut line) {
        let v : Vec<i64> = line.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        if v.len() == 2 {
            return Some((v[0], v[1]))
        }
    }
    None
}

fn main() {
    while let Some((a, b)) = read_numbers() {
        println!("{}", (a-b).abs());
    }
}
