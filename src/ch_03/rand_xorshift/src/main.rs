use std::time::SystemTime;

fn main() {
    let mut seed = rand_init();

    for _ in 0..100 {
        let x = rand(&mut seed, 1, 6);
        let y = rand(&mut seed, 1, 6);
        let z = rand(&mut seed, 1, 6);

        println!("seed: {}, x: {}, y: {}, z: {}", seed, x, y, z);
    }
}

fn rand_init() -> u32 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u32
}

fn rand(seed: &mut u32, start: u32, end: u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;

    return *seed % (end - start + 1) + start;
}