#[cfg(test)]
mod tests {
    #[test]
    fn test_array() {
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];
        assert_eq!(a1, a2);

        let a3 = ["apple".to_string(), "banana".to_string(), "cherry".to_string()];
        let a4 = [String::from("apple"), String::from("banana"), String::from("cherry")];
        assert_eq!(a3, a4);
    }

    #[test]
    fn test_vec() {
        let v1 = vec!["apple", "banana", "cherry"];
        let mut v2: Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        v2.push("cherry");
        assert_eq!(v1, v2);
    }
}
