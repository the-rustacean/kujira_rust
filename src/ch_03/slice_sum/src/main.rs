fn main() {

    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("sum a: {}", slice_sum(&a[..5]));

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("sum v: {}", slice_sum(&v[..5]));
}


fn slice_sum(slice: &[i64]) -> i64 {
    let mut total = 0;

    for i in slice {
        total += i;
    }

    total
}
