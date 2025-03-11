use std::collections::HashMap;

fn main() {

    // let name = "봉긋달";
    let months = ["해오름달", "시샘달", "꽃내음달", "잎새달", "푸른달", "누리달",
        "빗방울달", "타오름달", "거둠달", "온누리달", "눈마중달", "매듭달"];

    let mut months_map: HashMap<&str, usize> = HashMap::new();

    for (i, v) in months.iter().enumerate() {
        months_map.insert(v, i + 1);
    }

    println!("누리달 = {}월", months_map["누리달"]);


    for m in months {
        println!("{} = {}", m, months_map[m]);
    }


    for m in months.iter() {
        println!("{} = {}", m, months_map[m]);
    }

}
