use crate::util::{count_digits, is_prime, Primes};
use num::pow;

fn is_left_truncatable_prime(mut n: i64) -> bool {
    let mut d = pow(10, count_digits(n));
    loop {
        n %= d;
        if is_prime(n) {
            if n < 10 {
                return true;
            }
            d /= 10;
        } else {
            return false;
        }
    }
}

fn is_right_truncatable_prime(mut n: i64) -> bool {
    loop {
        n /= 10;
        if is_prime(n) {
            if n < 10 {
                return true;
            }
        } else {
            return false;
        }
    }
}

pub fn sol() -> i64 {
    Primes::new()
        .skip(4)
        .filter(|p| is_left_truncatable_prime(*p) && is_right_truncatable_prime(*p))
        .take(11)
        .sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 748317);
}

#[test]
fn test_r() {
    is_right_truncatable_prime(353);
}
