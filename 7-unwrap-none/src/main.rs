fn main() {
    // Lots of functions return an Option<T> or Result<T,Err>
    // Both Option and Result has an unwrap() method which returns the
    // actual value, but it is a panic to unwrap() None or an error.
    let v = vec!(3, 5, 7);
    let i = v.get(1).unwrap();
    println!("i is {}", i);
    // Getting an out of bonds element is ok, but returns None.
    // Unwrapping None is a panic.
    let i = v.get(7).unwrap();
    println!("i is {}", i);

    // Here is one proper way to do it:
    let index = 17;
    if let Some(i) = v.get(index) {
        println!("#{} is {}", index, i);
    } else {
        println!("There is no #{}", index);
    }
}
