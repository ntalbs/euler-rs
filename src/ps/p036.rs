use crate::util::is_palindrome;

fn is_binary_palindrome(n: i64) -> bool {
    let bin = format!("{:b}", n);
    let rev = bin.chars().rev().collect::<String>(); 
    bin.eq(&rev)
}

pub fn sol() -> i64 {
    (1..1_000_000)
        .filter(|n| is_palindrome(*n) && is_binary_palindrome(*n))
        .sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 872187);
}
