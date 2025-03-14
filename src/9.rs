use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "apple");
    map.insert(2, "banana");
    map.insert(3, "cherry");
    println!("{:?}", map);
}
