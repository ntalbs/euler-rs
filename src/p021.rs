use crate::util::{factorize, pow};

fn aliquot_sum(n: u64) -> u64 {
    if n <= 1 {
        return 0;
    }
    factorize(n).iter()
        .map(|(p, e)| (pow(*p, *e + 1) - 1) / (p - 1))
        .product::<u64>() - n
}

fn is_amicable(a: u64) -> bool {
    let b = aliquot_sum(a);
    (a != b) && (a == aliquot_sum(b))
}

pub fn sol() -> u64 {
    (1..10_000).filter(|n| is_amicable(*n)).sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 31626);
}
