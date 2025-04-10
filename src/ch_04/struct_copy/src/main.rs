fn main() {
    let dave = Person {
        name: "Dave".to_string(),
        age: 30,
    };

    let allison = Person {
        name: "Allison".to_string(),
        ..dave
    };

    println!("{}, {}", dave.name, dave.age);
    println!("{}, {}", allison.name, allison.age);
}

struct Person {
    name: String,
    age: u32,
}
