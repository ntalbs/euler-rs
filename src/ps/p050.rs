use crate::util::Sieve;
use itertools::iproduct;
use lazy_static::lazy_static;

lazy_static! {
    static ref SIEVE: Sieve = Sieve::new(1_000_000);
    static ref CSP: Vec<i64> = cummulative_prime_sum();
}

fn cummulative_prime_sum() -> Vec<i64> {
    let mut csp = Vec::new();
    let mut sum: i64 = 0;
    for i in 1..1_000_000 {
        if SIEVE.is_prime(i) {
            sum += i as i64;
            if sum > 1_000_000 {
                break;
            }
            csp.push(sum);
        }
    }
    csp
}

pub fn sol() -> i64 {
    iproduct!(CSP.iter().enumerate(), CSP.iter().enumerate())
        .filter(|((i, _a), (j, _b))| i < j)
        .map(|((i, a), (j, b))| (j - i, b - a))
        .filter(|(_, delta)| SIEVE.is_prime(*delta as usize))
        .max_by_key(|&(_, delta)| delta)
        .unwrap()
        .1
}

#[test]
fn test() {
    assert_eq!(sol(), 997651);
}
