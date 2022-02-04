use crate::util::{is_prime, Primes};
use itertools::iproduct;

fn f(a: i64, b: i64) -> Box<dyn Fn(i64) -> i64>  {
    Box::new(move |n| n*n + a*n + b)
}

fn count_primes(a: i64, b: i64) -> usize {
    (0..)
        .map(|n| f(a, b)(n))
        .take_while(|x| is_prime(*x))
        .count()
}

pub fn sol() -> i64 {
    let primes:Vec<i64> = Primes::new().take_while(|p| *p < 1_000).collect();
    iproduct!(-999..1_000, primes)
        .filter(|(a, b)| is_prime(1 + a + b))
        .map(|(a, b)| (a * b, count_primes(a, b)))
        .max_by_key(|(_, cnt)| *cnt)
        .unwrap()
        .0
}

#[test]
fn test() {
    assert_eq!(sol(), -59231);
}
