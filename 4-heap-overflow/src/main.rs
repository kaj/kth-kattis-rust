use std::collections::BTreeMap;

/// Note: It is probably a good idea to set a smalish memory limit
/// before running this ...
fn main() {
    let mut data = BTreeMap::new();
    for i in 0..100_000 {
        data.insert(i, vec![17; i]);
    }
    println!("Ok!?!");
}
