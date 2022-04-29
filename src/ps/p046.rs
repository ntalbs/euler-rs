use itertools::iproduct;
use std::collections::HashSet;

use crate::util::*;

const LIMIT: i64 = 10_000;

fn primes() -> HashSet<i64> {
    Primes::new().take_while(|&p| p <= LIMIT).collect()
}

fn square_nums() -> HashSet<i64> {
    (1..).map(|n| n * n).take_while(|&n| n <= LIMIT).collect()
}

fn goldbach_nums() -> HashSet<i64> {
    iproduct!(&primes(), &square_nums())
        .map(|(p, s)| p + 2 * s)
        .collect()
}

fn odd_composite_nums() -> HashSet<i64> {
    let odds: HashSet<i64> = (3..=LIMIT).step_by(2).collect();
    odds.difference(&primes()).cloned().collect()
}

pub fn sol() -> i64 {
    *odd_composite_nums()
        .difference(&goldbach_nums())
        .min()
        .unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 5777);
}
