use rand::prelude::*;

fn main() {
    let mut numbers = [0; 75];

    for x in 1..=75 {
        numbers[x - 1] = x;
    }

    numbers.shuffle(&mut rand::rng());

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                print!("  *,");
            } else {
                print!("{:3},", numbers[i]);
            }
        }
        println!("");
    }
}
