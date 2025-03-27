fn main() {
    let text = "지혜는 무기보다 가치가 있다.";

    let sub: String = text.chars().take(2).collect();

    println!("{}", sub);

    let text_chars: Vec<char> = text.chars().collect();

    let sub_chars = &text_chars[4..6];

    let sub: String = sub_chars.into_iter().collect();

    println!("{}", sub);
}
