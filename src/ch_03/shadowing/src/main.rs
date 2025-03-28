fn main() {
    {
        let mut v = 300;
        v = v + 3;
        println!("{}", v);
    }
    {
        let v = 300;
        let v = v + 3;
        println!("{}", v);
    }
}
