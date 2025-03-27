fn main() {

    let text = "😁😻🙈🙅🙆🙋";

    // println!("{}", text);

    let mut sub = String::new();

    for (i, c) in text.chars().enumerate() {
        if i < 2 {
            sub.push(c);
            continue;
        }
        break;
    }

    println!("{}", sub);

    let mut sub = String::new();

    for (i, c) in text.chars().enumerate() {
        if i >= 3 && i <= 4 {
            sub.push(c);
        }
    }

    println!("{}", sub);
}
