fn main() {
    for n in 1..=50 {

        if n % 3 == 0 {
            print!("A");
        } else if n % 10 == 3 {
            print!("A");
        } else if n / 30 == 1 && n % 30 < 10 {
            print!("A");
        } else {
            print!("{}", n);
        }

        if n % 3 == 0 {
            println!("");
        } else {
            print!("\t");
        }
    }
}
