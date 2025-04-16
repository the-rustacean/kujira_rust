fn main() {
    println!("{}", add(10, 20));
    println!("{}", add(10.5, 20.5));
    println!("{}", add::<u32>(10, 20));
    println!("{}", add::<f32>(10.5, 20.5));
    // println!("{}", add('a', 'b'));
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
