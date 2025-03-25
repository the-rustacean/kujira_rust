fn main() {
    let s = "안녕하세요, Tom.";

    println!("0 {}", s.chars().nth(0).unwrap());
    println!("1 {}", s.chars().nth(1).unwrap());
    println!("2 {}", s.chars().nth(2).unwrap());
    println!("3 {}", s.chars().nth(3).unwrap());
    println!("4 {}", s.chars().nth(4).unwrap());
    println!("5 {}", s.chars().nth(5).unwrap());
    println!("6 {}", s.chars().nth(6).unwrap());
    println!("7 {}", s.chars().nth(7).unwrap());
    println!("8 {}", s.chars().nth(8).unwrap());
    println!("9 {}", s.chars().nth(9).unwrap());
    println!("10 {}", s.chars().nth(10).unwrap());
    // println!("11 {}", s.chars().nth(11).unwrap());

    println!("{}", &s[..3]);
    println!("{}", &s[3..6]);
    println!("{}", &s[6..9]);
    println!("{}", &s[9..12]);
    println!("{}", &s[12..15]);
    println!("{}", &s[15..16]);
    println!("{}", &s[16..17]);
    println!("{}", &s[17..18]);
    println!("{}", &s[18..19]);
    println!("{}", &s[19..20]);
    println!("{}", &s[20..21]);
}
