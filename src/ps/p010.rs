#![allow(dead_code)]
#![allow(unused_variables)]

use crate::util::{Primes, Sieve};

const LIMIT: i64 = 2_000_000;

/// use Primes iterator
fn sol_1() -> i64 {
    Primes::new().take_while(|x| x <= &LIMIT).sum()
}

/// use sieve
fn sol_2() -> i64 {
    let sieve = Sieve::new(LIMIT as usize);
    let mut sum = 0;
    for i in 1..LIMIT as usize {
        if sieve.is_prime(i) {
            sum += i;
        }
    }
    sum as i64
}

pub fn sol() -> i64 {
    sol_1()
}

#[test]
fn test() {
    assert_eq!(sol_1(), 142913828922);
    assert_eq!(sol_2(), 142913828922);
}
