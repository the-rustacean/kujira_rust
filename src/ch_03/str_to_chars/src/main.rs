fn main() {

    let pr = "러스트에서, 각각의 값은 소유자(owner)가 정해져 있습니다.";

    for c in pr.chars() {
        print!("[{}]", c);
    }

    println!("\nchars = {}", pr.chars().count());

    let pr_chars: Vec<char> = pr.chars().collect();
    println!("Vec<char> : {:?}", pr_chars);

    for c in pr_chars.iter() {
        print!("({})", c);
    }

    println!("\nchars = {}", pr_chars.len());
}
