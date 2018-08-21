use std::fs;

fn main() {
    // Lots of functions return an Option<T> or Result<T,Err>
    // Both Option and Result has an unwrap() method which returns the
    // actual value, but it is a panic to unwrap() None or an error.
    let filename = "nosuchfile";
    let meta = fs::metadata(filename).unwrap();
    println!("File {} is {} bytes", filename, meta.len());

    // Here is one proper way to do it:
    match fs::metadata(filename) {
        Ok(meta) => println!("File {} is {} bytes", filename, meta.len()),
        Err(err) => println!("Error looking at {}: {}", filename, err),
    };
}
