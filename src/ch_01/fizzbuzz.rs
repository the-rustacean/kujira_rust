fn main() {
    for n in 1..=100 {

        if n % 3 == 0 && n % 5 == 0 {
            print!("FizzBuzz");
        } else if n % 3 == 0 {
            print!("Fizz");
        } else if n % 5 == 0 {
            print!("Buzz");
        } else {
            print!("{}", n);
        }

        if n % 5 == 0 {
            println!("");
        } else {
            print!("\t");
        }
    }
}
