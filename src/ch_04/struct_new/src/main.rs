fn main() {
    let lee = Person {
        name: "Lee".to_string(),
        age: 10,
    };

    let sue = Person::new("Sue".to_string(), 12);

    lee.print();
    sue.print();
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}
