use itertools::iproduct;
use num::{bigint::ToBigUint, BigUint};
use crate::util::digits;

pub fn sol() -> i64 {
    iproduct!(1..100, 1..100)
        .map(|(a, b)| a.to_biguint().unwrap().pow(b))
        .map(digits::<BigUint, i64>)
        .map(|ds| ds.iter().sum())
        .max()
        .unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 972);
}