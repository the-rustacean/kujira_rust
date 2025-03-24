fn main() {
    let banana = ("바나나", 300);
    let orange = ("오렌지", 900);

    let total = banana.1 + orange.1;

    print_tuple(&banana);
    print_tuple(&orange);

    println!("합계는 {}원 입니다.", total);
}

fn print_tuple(item: &(&str, u16)) {
    println!("{}를 {}원에 구입", item.0, item.1)
}
