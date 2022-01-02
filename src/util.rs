use std::collections::LinkedList;

/// Returns the greatest common divisor of m and n.
pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

/// Returns the least common multiplier of m and n.
pub fn lcm(m: u64, n: u64) -> u64 {
    (m * n) / gcd(m, n)
}

pub struct Primes {
    primes: LinkedList<u32>,
    next: u32,
}

impl Primes {
    pub fn new() -> Self {
        Primes {
            primes: LinkedList::new(),
            next: 2,
        }
    }
    fn is_prime(&self, n: u32) -> bool {
        for p in &self.primes {
            if n % p == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for Primes {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        loop {
            if self.is_prime(self.next) {
                let current = self.next;
                self.primes.push_back(self.next);
                self.next += 1;
                return Some(current)
            }
            self.next += 1;
        }
    }
}

pub struct Sieve {
    primes: Vec<bool>,
    limit: usize,
}

impl Sieve {
    pub fn new(limit: usize) -> Self {
        let mut sieve = Sieve {
            primes: Vec::with_capacity(limit/2),
            limit,
        };
        sieve.init();
        sieve
    }

    fn init(&mut self) {
        self.primes.push(false);
        for _ in 1..self.primes.capacity() {
            self.primes.push(true);
        }

        for n in (3..self.limit).step_by(2) {
            for m in (3..).step_by(2) { // skip even factors
                if m * n >= self.limit {
                    break;
                }
                let i = (m * n - 1) / 2;     // array index for multiples of n
                self.primes[i] = false;
            }
        }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        match n {
            0 | 1 => false,                // 0, 1 is not prime
            2 => true,                     // 2 is prime
            _ => if n % 2 == 0 {
                false                      // even numbers other than 2 are not prime
            } else {
                self.primes[(n - 1) / 2]   // for odd numbers bigger than 2, check sieve
            }
        }
    }

    pub fn print(&self) {
        for i in 0..self.primes.len() {
            println!("i={}, n={}, is_prime? {}, is_prime? {}", i, 2*i+1, self.primes[i], self.is_prime(2*i+1));
        }
    }
}
