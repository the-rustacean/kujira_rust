fn main() {
    let mut point = Point::new(10, 10);
    println!("{:?}", point);


    point.add(Point { x: 22, y: 22 });
    println!("{:?}", point);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> where T: std::ops::AddAssign {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}
