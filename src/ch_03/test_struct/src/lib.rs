#[derive(Debug, PartialEq)]
struct GItem {
    name: String,
    price: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item() {
        let apple1 = GItem {
            name: String::from("apple"),
            price: 2400,
        };
        let mut apple2 = GItem {
            name: "apple".to_string(),
            price: 1,
        };
        apple2.price = 2400;

        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);

        assert_eq!(apple1, apple2);
    }
}
