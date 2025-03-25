fn main() {
    let hong = Body {
        weight: 80.0,
        height: 165.0,
    };

    let park = Body {
        weight: 65.0,
        height: 170.0,
    };

    println!("홍 = {:.1}", cal_bmi(&hong));
    println!("박 = {:.1}", cal_bmi(&park));
}

struct Body {
    weight: f64,
    height: f64,
}

fn cal_bmi(who: &Body) -> f64 {
    let h = who.height / 100.0;
    who.weight / h.powf(2.0)
}

