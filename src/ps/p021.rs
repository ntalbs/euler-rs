use crate::util::aliquot_sum;

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
