fn main() {
    let coin_500 = 10;
    let coin_100 = 3;
    let coin_50 = 10;

    let change = 3950;

    for count_500 in 0..=coin_500 {
        for count_100 in 0..=coin_100 {
            for count_50 in 0..=coin_50 {
                let sum = (500 * count_500) + (100 * count_100) + (50 * count_50);

                if sum == change {
                    println!("500원: {} 개, 100원: {} 개, 50원: {} 개", count_500, count_100, count_50);
                }
             }
        }
    }
}
