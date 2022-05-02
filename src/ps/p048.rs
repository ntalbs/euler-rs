use num::{bigint::ToBigInt, BigInt, Integer, ToPrimitive};

fn n_pow_n(n: i64) -> BigInt {
    n.to_bigint().unwrap().pow(n as u32)
}

pub fn sol() -> i64 {
    (1..=1000)
        .map(n_pow_n)
        .sum::<BigInt>()
        .mod_floor(&10_000_000_000_i64.to_bigint().unwrap())
        .to_i64()
        .unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 9110846700);
}
