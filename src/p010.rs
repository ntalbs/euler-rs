#![allow(dead_code)]
#![allow(unused_variables)]

use crate::util::{Primes, Sieve};

const LIMIT: u64 = 2_000_000;

/// use Primes iterator
fn sol_1() -> u64 {
    Primes::new().take_while(|x| x <= &LIMIT).sum()
}

/// use sieve
fn sol_2() -> u64 {
    let sieve = Sieve::new(LIMIT as usize);
    let mut sum = 0;
    for i in 1..LIMIT as usize {
        if sieve.is_prime(i) {
            sum += i;
        }
    }
    sum as u64
}

pub fn sol() -> u64 {
    sol_1()
}

#[test]
fn test() {
    assert_eq!(142913828922, sol_1());
    assert_eq!(142913828922, sol_2());
}
