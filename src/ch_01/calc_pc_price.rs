fn main() {
    let pc_price = 980_000_f64;
    let a_shipping_charge = 12_000_f64;
    let a_sale = 0.2_f64;
    let b_shipping_charge = 0_f64;
    let b_sale = 0.1_f64;

    let a_shop_price = pc_price * (1.0 - a_sale) + a_shipping_charge;
    let b_shop_price = pc_price * (1.0 - b_sale) + b_shipping_charge;

    println!("A 쇼핑몰 = {} 원", a_shop_price);
    println!("B 쇼핑몰 = {} 원", b_shop_price);
}
