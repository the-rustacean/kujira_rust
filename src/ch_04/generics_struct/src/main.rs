fn main() {
    let point_i = Point { x: 10, y: 20 };
    let point_f = Point { x: 10.5, y: 20.4 };

    println!("{:?}", point_i);
    println!("{:?}", point_f);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
