use std::env;

fn main() {

    let args = env::args();
    println!("{:?}", args);

    println!("use Args");
    for arg in env::args() {
        println!("{}", arg);
    }

    println!("use Vec<String>");
    for arg in env::args().collect::<Vec<String>>() {
        println!("{}", arg);
    }
}
