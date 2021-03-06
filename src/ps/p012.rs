use crate::util::factorize;

fn triangular_number(n: i64) -> i64 {
    n * (n + 1) / 2
}

#[rustfmt::skip]
fn count_divisors(n: i64) -> i64 {
    factorize(n)
        .values()
        .map(|x| x + 1)
        .product()
}

pub fn sol() -> i64 {
    (1..)
        .map(triangular_number)
        .find(|n| count_divisors(*n) >= 500)
        .unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 76576500);
}
