use itertools::iproduct;
use num::rational::Ratio;

fn c(n: u128, r: u128) -> u128 {
    if r > n {
        return 0;
    }
    let nu = (n - r + 1)..=n;
    let de = 1..=r;
    *nu.zip(de)
        .map(|(a, b)| Ratio::new(a, b))
        .reduce(|a, b| a * b)
        .unwrap()
        .numer()
}

pub fn sol() -> i64 {
    iproduct!(1..=100_u128, 1..=100_u128)
        .map(|(n, r)| c(n, r))
        .filter(|v| v >= &1_000_000)
        .count()
        .try_into()
        .unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 4075);
}

#[test]
fn test_c() {
    assert_eq!(c(1, 1), 1);
    assert_eq!(c(1, 2), 0);
    assert_eq!(c(2, 1), 2);
}
