use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert("A", 30);
    map.insert("B", 50);
    map.insert("C", 90);

    let d = match map.get("D") {
        Some(v) => *v,
        None => 1000,
    };

    println!("{}", d);
}
