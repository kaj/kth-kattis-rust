
/// Return fibonacci number #n.
fn fib(n: u32) -> u32 {
    if n > 2 {
        fib(n-1) + fib(n-2)
    } else {
        1
    }
}

fn main() {
    let i = 100;
    println!("Fibonacci #{} is {}", i, fib(i));
}
