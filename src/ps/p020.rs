use crate::util::{digits, factorial};

pub fn sol() -> i64 {
    digits(factorial(100)).iter().sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 648);
}
