use std::io::stdin;

fn main() {

    let height_cm = input("키(cm) : ");
    let weight = input("몸무게(kg) : ");

    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);

    println!("BMI = {:.1}", bmi);

    if bmi < 18.5 { println!("저체중"); }
    else if bmi < 23.0 { println!("정상"); }
    else if bmi < 25.0 { println!("비만전단계"); }
    else if bmi < 30.0 { println!("1단계 비만"); }
    else if bmi < 35.0 { println!("2단계 비만"); }
    else { println!("3단계 비만"); }
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::from("");
    stdin().read_line(&mut s).expect("입력 에러");
    return s.trim().parse().expect("숫자가 아닙니다.");
}
