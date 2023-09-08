use crate::util::{is_palindrome, reverse};

fn is_lychrel(mut n: u128) -> bool {
    for _ in 1..=50 {
        let sn = n + reverse(n);
        if is_palindrome(sn) {
            return false;
        }
        n = sn;
    }
    true
}

pub fn sol() -> i64 {
    (10..=10000).filter(|&x| is_lychrel(x)).count() as i64
}

#[test]
fn test() {
    assert_eq!(sol(), 249);
}
