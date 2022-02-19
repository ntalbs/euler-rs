use std::collections::HashSet;

use itertools::iproduct;
use num::{bigint::ToBigUint, pow, BigUint};

pub fn sol() -> i64 {
    let set: HashSet<BigUint> = iproduct!(2..=100, 2..=100)
        .map(|(a, b)| pow(a.to_biguint().unwrap(), b as usize))
        .collect();
    set.len() as i64
}

#[test]
fn test() {
    assert_eq!(sol(), 9183);
}
