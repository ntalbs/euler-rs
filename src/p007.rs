use crate::util::Primes;

pub fn sol() -> u64 {
    Primes::new().skip(10000).next().unwrap()
}

#[test]
fn test() {
    assert_eq!(104743, sol());
}
