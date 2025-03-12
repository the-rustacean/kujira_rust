fn main() {
    for i in  2..=20 {
        println!("{}", fib(i));
    }
}

fn fib(n: i64) -> i64 {
    if n == 1 {
        return 0;
    } else  if n == 2 {
        return 1;
    } else {
        return fib(n - 2) + fib(n - 1);
    }
}
