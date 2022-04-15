use crate::util::Primes;

pub fn sol() -> i64 {
    Primes::new().nth(10000).unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 104743);
}
