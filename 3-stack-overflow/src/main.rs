
fn foo(i: u8) -> u8 {
    if i < 3 {
        i * 2
    } else {
        foo(i)
    }
}

fn main() {
    println!("Returned {}?!?", foo(17));
}
