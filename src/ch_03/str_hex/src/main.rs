fn main() {
    let text = "성공하는 사람은 송곳처럼 어느 한 점을 향하여 일한다.";

    for (i, c) in text.bytes().enumerate() {

        if i % 16 == 0 {
            print!("{:08x}:", i);
        }

        if i % 4 == 3 {
            print!(" {:02x} |", c);
        } else {
            print!(" {:02x} ", c);
        }

        if i % 16 == 15 {
            println!();
        }
    }
    println!();
}
