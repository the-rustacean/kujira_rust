use std::time::{SystemTime, UNIX_EPOCH};

static mut SEED: u32 = 0;

fn main() {
    for _ in 0..10 {
        let random_number = unsafe { rand_global(1, 10) };
        println!("{}", random_number);
    }
    println!("Rand End!");
}

unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {
        let epoc = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        SEED = epoc.as_millis() as u32;
    }
    
    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;

    return SEED % (end - start) + start;
}