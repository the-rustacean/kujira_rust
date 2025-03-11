use std::fs;

fn main() {

    let a_file = "./fizzbuzz_python.txt";
    let b_file = "./fizzbuzz_rust.txt";

    let a_text = fs::read_to_string(a_file).unwrap();
    let b_text = fs::read_to_string(b_file).unwrap();

    let a_text = a_text.trim();
    let b_text = b_text.trim();

    if a_text == b_text {
        println!("ok");
    } else {
        println!("ng");
    }
}
