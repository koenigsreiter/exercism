use std::iter::Iterator;

struct PrimeStream {
    primes: Vec<u32>,
    current: u32,
}

impl PrimeStream {
    pub fn new() -> PrimeStream {
        PrimeStream {
            primes: vec![],
            current: 2,
        }
    }
}

impl Iterator for PrimeStream {
    type Item = u32;

    fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
        for i in self.current..u32::max_value() {
            if self.primes.iter().all(|x| i % x != 0) {
                self.primes.push(i);
                self.current = i + 1;
                return Some(i);
            }
        }
        None
    }
}

pub fn nth(n: u32) -> u32 {
    let mut ps = PrimeStream::new();
    ps.nth(n as usize).unwrap()
    //unimplemented!("What is the 0-indexed {}th prime number?", n)
}
