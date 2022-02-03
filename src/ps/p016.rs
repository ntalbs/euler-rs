use crate::util::digits;
use num::{bigint::ToBigUint, BigUint};

pub fn sol() -> i64 {
    let two = 2.to_biguint().unwrap();
    let mut x: BigUint = 1.to_biguint().unwrap();

    for _ in 1..=1000 {
        x = x * &two;
    }

    let digits = digits(x);
    digits.iter().sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 1366);
}
