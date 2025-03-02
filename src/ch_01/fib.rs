fn main() {
    println!("hi");

    let mut first = 1;
    let mut second = 1;

    println!("{}", first);
    println!("{}", second);

    for _ in 0..30 {
        let sum = first + second;
        first = second;
        second = sum;
        println!("{}", sum);
    }
}
