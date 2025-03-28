fn main() {
    let contents = "제주도의 특산품 중 귤은 겨울에 많이 먹을 수 있다.";

    str_find(&contents, "제주도");

    str_find(&contents, "바나나");
}

fn str_find(contents: &str, query: &str) {
    match contents.find(query) {
        Some(i) => println!("{} = {}B", query, i),
        None => println!("'{}'라는 단어는 없습니다.", query),
    }
}
