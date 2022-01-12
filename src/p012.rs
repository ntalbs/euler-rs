use crate::util::factorize;

fn triangular_number(n: u64) -> u64 {
    n * (n + 1) / 2
}

#[rustfmt::skip]
fn count_divisors(n: u64) -> u64 {
    factorize(n)
        .values()
        .map(|x| x + 1)
        .product()
}

/// SLOW!!!
/// Probably Primes iterator needs to be cached
/// Reduce loop in factorize 2 .. sqrt(n)
pub fn sol() -> u64 {
    (1..)
        .map(triangular_number)
        .skip_while(|n| count_divisors(*n) < 500)
        .next()
        .unwrap()
}
