use crate::util::Primes;

pub fn sol() -> i64 {
    Primes::new().skip(10000).next().unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 104743);
}
