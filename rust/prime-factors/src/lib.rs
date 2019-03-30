use std::iter::Iterator;

struct PrimeStream {
    primes: Vec<u64>,
    current: u64,
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
    type Item = u64;

    fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
        for i in self.current..u64::max_value() {
            if self.primes.iter().all(|x| i % x != 0) {
                self.primes.push(i);
                self.current = i + 1;
                return Some(i);
            }
        }
        None
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut rem = n;
    match n {
        1 => return vec![],
        2 => return vec![2],
        _ => {
            let mut ret_vec = PrimeStream::new()
                .take_while(|prime| (prime * prime) <= n)
                .fold(vec![], |mut prime_vector, prime| {
                    while (rem % prime) == 0 {
                        rem = rem / prime;
                        prime_vector.push(prime);
                    }
                    prime_vector
                });
            if rem != 1 {
                ret_vec.push(rem);
            }
            ret_vec
        }
    }
    //unimplemented!("This should calculate the prime factors of {}", n)
}
