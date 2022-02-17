use num::pow;

use crate::util::Sieve;

fn cycle(n: usize, p: usize) -> usize {
    let front = n / 10;
    let last = n % 10;
    last * pow(10, p) + front
}

fn is_circular_prime(mut n: usize, sieve: &Sieve) -> bool {
    if !sieve.is_prime(n) {
        return false;
    }

    let p = (n as f64).log(10.0).floor() as usize;
    for _ in 0..p {
        n = cycle(n, p);
        if !sieve.is_prime(n) {
            return false;
        }
    }
    true
}

pub fn sol() -> i64 {
    let sieve = Sieve::new(1_000_000);

    (2..1_000_000)
        .filter(|n| is_circular_prime(*n, &sieve))
        .count() as i64
}

#[test]
fn test() {
    assert_eq!(sol(), 55);
}
