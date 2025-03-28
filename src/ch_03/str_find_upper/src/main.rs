fn main() {
    let contents = format!("{}{}", "abcdefg", "hijklmnop");

    str_find_upper(&contents, 'H');

    str_find_upper(&contents, 'S');
}

fn str_find_upper(contents: &str, ch: char) {
    match contents.find(|c:char| c.to_ascii_uppercase() == ch) {
        Some(i) => println!("{} = {}B", ch, i),
        None => println!("None of '{}'", ch),
    }
}
