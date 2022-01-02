use std::collections::LinkedList;

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