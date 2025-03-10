use std::collections::HashMap;

fn main() {
    let mut votes = HashMap::from([
        ("A", 0),
        ("B", 0),
        ("C", 0),
    ]);

    const V_DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C";

    for ch in V_DATA.split(",") {
        votes.insert(ch, votes[ch] + 1);
    }

    for k in ["A", "B", "C"] {
        println!("{}: {:>2}", k, votes[k]);
    }
}
