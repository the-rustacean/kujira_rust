fn main() {

    let mut greet = "hello".to_string();
    println!("{}", greet);

    greet.push_str(", world");
    println!("{}", greet);


    let mut name = "David";
    println!("{}", name);

    name = "Joe";
    println!("{}", name);

    let mut age = 128;
    println!("{}", age);

    age = 256;
    println!("{}", age);
}
