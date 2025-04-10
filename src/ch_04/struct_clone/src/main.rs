fn main() {
    let alex = Person {
        name: "Alex".to_string(),
        age: 20,
    };

    let emily = alex.clone();

    println!("{}, {}", alex.name, alex.age);
    println!("{}, {}", emily.name, emily.age);
}

#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
}
