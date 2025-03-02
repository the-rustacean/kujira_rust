// 1392 태조
// 1399 정종
// 1401 태종
// 1419 세종

fn main() {
    for n in 1392..=1450 {
        print!("서력 {} 년 = ", n);

        let name: &str;
        let era: u32;

        if n < 1399 {
            name = "태조";
            era = n - 1392;
        } else if n < 1401 {
            name = "정종";
            era = n - 1399;
        } else if n < 1419 {
            name = "태종";
            era = n - 1401;
        } else {
            name = "세종";
            era = n - 1419;
        }

        if era == 0 {
            println!("{} 원년", name);
        } else {
            println!("{} {} 년", name, era);
        };
    }
}
