use std::env;

fn main() {

    let mut total = 0.0;

    for arg in env::args() {
        // println!("{}", arg);
        match arg.parse::<f64>() {
            Ok(v) => total += v,
            Err(_) => (),
        };
    }

    println!("{}", total);
}
