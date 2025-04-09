#[cfg(test)]
mod tests {
    #[test]
    fn test_calc1() {
        assert_eq!(100 * 2, 200);
        assert_eq!((1 + 2) * 3, 9);
        assert_eq!(1 + 2 * 3, 7);
    }

    #[test]
    fn test_calc2() {
        assert_eq!(2 * 3, 6);
        assert_eq!(2 * 4, 8);
    }
}
