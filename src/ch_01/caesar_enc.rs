fn main() {
    let enc = encrypt("I LOVE RUST.", 3);
    let dec = encrypt(&enc, -3);

    println!("{} => {}", enc, dec);
}

fn encrypt(sentence: &str, shift: i16) -> String {

    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    let mut result = String::new();

    for ch in sentence.chars() {
        
        let mut code_ch = ch as i16;

        if code_ch >= code_a && code_ch <= code_z {
            code_ch = (code_ch - code_a + shift + 26) % 26 + code_a;
        }

        result.push((code_ch as u8) as char);
    }

    result
}
