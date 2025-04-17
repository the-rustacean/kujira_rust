fn main() {
    for i in 1..=10 {
        if i % 2 != 0 {
            println!("{}", i);
        }
    }

    let numbers = [1, 3, 5, 7, 9];

    println!("len: {}", numbers.len());

    for n in numbers {
        println!("{}", n);
    }

    println!("len: {}", numbers.len());
}
