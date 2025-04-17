fn main() {

    let fruits = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Pineapple".to_string(),
    ];

    for fruit in fruits.iter() {
        println!("{}", fruit);
    }

    println!("len: {}", fruits.len());
}
