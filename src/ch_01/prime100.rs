fn main() {
    let mut primes = [0; 100];

    make_prime(&mut primes);

    println!("{:?}", primes);
}

fn is_prime(number: u64) -> bool {
    for divisor in 2..number {
        if number % divisor == 0 {
            return false;
        }
    }

    true
}

fn make_prime(list: &mut [u64]) {
    let mut number: u64 = 2;
    let mut count: usize = 0;

    while count < list.len() {
        if is_prime(number) {
            list[count] = number;
            count = count + 1;
        }

        number = number + 1;
    }
}