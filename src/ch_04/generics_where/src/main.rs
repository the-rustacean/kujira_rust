fn main() {
    println!("{}", x2(3));
    println!("{}", x2(0.4));
    println!("{}", x2::<f64>(5.4));
}

fn x2<T>(n: T) -> T 
    where T: std::ops::Add<Output = T> + Copy
{
    n + n
}
