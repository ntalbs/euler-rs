use crate::util::{is_prime, Sieve};
use itertools::iproduct;
use lazy_static::lazy_static;

lazy_static! {
    static ref SIEVE: Sieve = Sieve::new(10_000);
}

fn concat_nums(a: i64, b: i64) -> i64 {
    format!("{a}{b}").parse::<i64>().unwrap()
}

fn is_concat_prime(a: i64, b: i64) -> bool {
    is_prime(concat_nums(a, b)) && is_prime(concat_nums(b, a))
}

fn is_any_two_concat_prime(ps: &[i64], p: i64) -> bool {
    for x in ps {
        if !is_concat_prime(*x, p) {
            return false;
        }
    }
    true
}

pub fn sol() -> i64 {
    let ps1: Vec<i64> = SIEVE.primes().iter().map(|p| *p as i64).collect();

    let ps2 = iproduct!(ps1.iter(), ps1.iter())
        .filter(|(p1, p2)| p1 < p2)
        .filter(|(p1, p2)| is_concat_prime(**p1, **p2))
        .map(|(p1, p2)| [*p1, *p2]);

    let ps3 = iproduct!(ps2, ps1.iter())
        .filter(|([p1, p2], p3)| p1 < p2 && p2 < p3)
        .filter(|([p1, p2], p3)| is_any_two_concat_prime(&[*p1, *p2], **p3))
        .map(|([p1, p2], p3)| [p1, p2, *p3]);

    let ps4 = iproduct!(ps3, ps1.iter())
        .filter(|([p1, p2, p3], p4)| p1 < p2 && p2 < p3 && p3 < p4)
        .filter(|([p1, p2, p3], p4)| is_any_two_concat_prime(&[*p1, *p2, *p3], **p4))
        .map(|([p1, p2, p3], p4)| [p1, p2, p3, *p4]);

    let mut ps5 = iproduct!(ps4, ps1.iter())
        .filter(|([p1, p2, p3, p4], p5)| p1 < p2 && p2 < p3 && p3 < p4 && p4 < p5)
        .filter(|([p1, p2, p3, p4], p5)| is_any_two_concat_prime(&[*p1, *p2, *p3, *p4], **p5))
        .map(|([p1, p2, p3, p4], p5)| [p1, p2, p3, p4, *p5]);

    ps5.next().unwrap().iter().sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 26033);
}
