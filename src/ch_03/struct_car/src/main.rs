fn main() {
    let car_1 = CarSpec {
        model: 3001,
        cc: 1500,
        color: 0xFF0000,
    };

    let car_2 = CarSpec {
        model: 3002,
        cc: 2500,
        color: 0x0000FF,
    };

    print_car(&car_1);
    print_car(&car_2);
}

fn print_car(car: &CarSpec) {
    println!("model: {}, {}cc, {:06x}", car.model, car.cc, car.color);
}

struct CarSpec {
    model: i32,
    cc: i32,
    color: i32,
}
