
fn foo(a: u64) -> u64 {
    if a < 3 {
        a
    } else {
        foo(a / 2) + foo(a + 1)
    }
}

fn main() {
    println!("Returned {}?!?", foo(17));
}
