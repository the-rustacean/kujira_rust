use std::{env, fs};

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).unwrap();

    // println!("{}", contents);

    let mut total = 0;

    for line in contents.split('\n') {
        println!("{}", line);

        match line.parse::<i32>() {
            Ok(v) => total += v,
            Err(_) => (),
        }
    }

    println!("{}", total);
}
