fn main() {
    let text = "내 자신에 대한 자신감을 잃으면 온 세상이 나의 적이 된다.";

    println!("{}", text);

    let text = str_replace(&text, "잃으면", "가지면");

    println!("{}", text);

    let text = str_replace(&text, "적이", "편이");

    println!("{}", text);
}

fn str_replace(contents: &str, from: &str, to: &str) -> String {

    contents.replace(from, to)
}
