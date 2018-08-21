mod greet;
use greet::get_greeting;

fn main() {
    println!("{}", get_greeting("world"));
}
