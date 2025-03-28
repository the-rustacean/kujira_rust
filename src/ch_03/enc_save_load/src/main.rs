use encoding_rs;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    let file_name = "test-euckr.txt";

    save_euckr(file_name, "맛있게 먹으면 0칼로리");

    let s = load_euckr(file_name);

    println!("{}", s);
}

fn save_euckr(filename: &str, text: &str) {
    let (enc, _, _) = encoding_rs::EUC_KR.encode(text);

    let but = enc.into_owned();

    let mut file = File::create(filename).expect("생성");

    file.write(&but[..]).expect("쓰기");
}

fn load_euckr(filename: &str) -> String {
    let buf = fs::read(filename).expect("읽기");

    let (dec, _, _) = encoding_rs::EUC_KR.decode(&buf);

    return dec.into_owned();
}
