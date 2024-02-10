use num::{bigint::ToBigInt, BigInt, ToPrimitive};

pub fn sol() -> i64 {
    let b = 2.to_bigint().unwrap();
    let e = 7_830_457.to_bigint().unwrap();
    let m = 10_000_000_000_u64.to_bigint().unwrap();
    let c = 28433.to_bigint().unwrap();

    let a: BigInt = (c * b.modpow(&e, &m) + 1) % m;
    a.to_i64().unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 8739992577);
}
