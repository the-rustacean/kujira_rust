fn main() {
    let ss1: &str = "The Rust Programming Language";

    let so1: String = String::from(ss1);
    let so2: String = ss1.to_string();

    let ss2: &str = &so1;
    let ss3: &str = so1.as_str();

    println!("so1 = {}, pointer: {:p}", so1, &so1);
    println!("so2 = {}, pointer: {:p}", so2, &so2);
    println!("ss1 = {}, pointer: {:p}", ss1, ss1);
    println!("ss2 = {}, pointer: {:p}", ss2, ss2);
    println!("ss3 = {}, pointer: {:p}", ss3, ss3);
}
