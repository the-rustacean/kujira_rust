use rand::prelude::*;

fn main() {
    let mut nums: Vec<i32> = (1..=75).collect();
    let mut rng = rand::rng();
    nums.shuffle(&mut rng);


    for i in 0..25 {
        if i == 12 {
            print!("  *,");
        } else {
            print!("{:3},", nums[i]);
        }
        if i % 5 == 4 {
            println!("");
        }
    }
}
