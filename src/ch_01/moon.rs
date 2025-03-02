fn main() {
    let moon = 384_400;
    let car = 80;
    let ktx = 300;

    println!("달까지 자동차로 {}일", moon / car / 24);
    println!("달까지 KTX로 {}일", moon / ktx / 24);
}
