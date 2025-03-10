fn main() {

    let s = "365o";

    let i: i32 = match s.parse() {
        Ok(v) => v,
        Err(_) => 0,
    };
    println!("i = {}", i);

    let s2 = "365";

    let i2: i32 = s2.parse().expect("not a number");
    println!("i2 = {}", i2);

    let s3 = "365o";

    let i3: i32 = s3.parse().unwrap();
    println!("i3 = {}", i3);
}
