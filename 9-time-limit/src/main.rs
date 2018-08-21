/// Return fibonacci number #n.
fn fib(n: u64) -> u64 {
    if n > 2 {
        fib(n - 1) + fib(n - 2)
    } else {
        1
    }
}

fn main() {
    // Takes 55 minutes of one core on my laptop, should be enough.
    let i = 60;
    println!("Fibonacci #{} is {}", i, fib(i));
}
