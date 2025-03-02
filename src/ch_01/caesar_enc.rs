fn main() {
    let enc = encrypt("I LOVE RUST.", 3);
    let dec = encrypt(&enc, -3);

    println!("{} => {}", enc, dec);
}

fn encrypt(sentence: &str, number: isize) -> String {

    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;

    for ch in sentence.chars() {
        
        let code_ch = ch as i16;

        let ch_next = if code_ch >= code_a && code_ch <= code_z {
            let code_next = code_ch + 3;
            char::from_u32(code_next as u32)
        } else {
            ch
        };

        println!("{}: {}", ch, ch_next);
    }

    String::from("hi")
}
