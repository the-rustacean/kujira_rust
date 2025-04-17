fn main() {
    let fib = FibIterator::new();

    for f in fib {
        print!("{} ", f);
    }

    let fib = FibIterator::new();
    for (i, n) in fib.enumerate() {
        if i >= 10 { break; }
        print!("{},", n);
    }
    println!();

    let fib = FibIterator::new();
    fib.take(10).for_each(|f| print!("{},", f));
    println!();
}

struct FibIterator {
    a: u32,
    b: u32,
}

impl FibIterator {
    fn new() -> Self {
        Self { a: 1, b: 1 }
    }
    fn calcurate(&mut self) -> Result<u32, &'static str> {
        match self.a.checked_add(self.b) {
            Some(v) => {
                self.a = self.b;
                self.b = v;
        
                return Ok(v);
            },
            None => {
                print!("\n{} + {} ", self.a, self.b);
                return Err("<- overflow occurred!!!");
            }
        }

    }
}

impl Iterator for FibIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.calcurate() {
                Ok(v) => {
                    return Some(v);
                },
                Err(e) => {
                    println!("{}", e);
                    break;
                },
            }
        }

        None
    }
}

