use crate::util::{from_digits, is_prime};
use itertools::Itertools;

fn find_prime(digits: &[i64]) -> Option<i64> {
    let len = digits.len();
    digits
        .iter()
        .rev()
        .copied()
        .permutations(len)
        .map(|ds| from_digits(&ds))
        .find(|n| is_prime(*n))
}

pub fn sol() -> i64 {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in (1..=9).rev() {
        return match find_prime(&digits[..i]) {
            Some(n) => n,
            None => continue,
        };
    }
    panic!("Solution not found.");
}

#[test]
fn test() {
    assert_eq!(sol(), 7652413);
}
