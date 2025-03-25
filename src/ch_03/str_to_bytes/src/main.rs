fn main() {

    let pr = "러스트에서, 각각의 값은 소유자(owner)가 정해져 있습니다.";

    for b in pr.bytes() {
        print!("{:2x} ", b);
    }

    println!("\nbyte = {}B", pr.len());
}
