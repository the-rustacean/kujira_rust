fn main() {

    echo("Hello, world!");
    echo("Hello, RUST!");
    
    let ss: &str = "Hello, 안녕하세요!";
    echo(ss);

//     let s = String::from("러스트");
//     echo(&s);
}

fn echo(s: &'static str) {
    println!("{}", s);
}
