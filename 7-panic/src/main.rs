fn main() {
    // The standard library (and other libraries) use a panic when
    // used in a cleaarly wrong way (most notably when unwrapping None
    // or Err).
    // A program can also call panic explicitly.
    panic!("Program called panic");
}
