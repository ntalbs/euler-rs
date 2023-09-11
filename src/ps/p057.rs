use crate::util::count_digits;
use num::{bigint::ToBigUint, rational::Ratio, BigUint};

fn conv(a: Ratio<BigUint>) -> Ratio<BigUint> {
    let one = Ratio::new(1.to_biguint().unwrap(), 1.to_biguint().unwrap());
    one.clone() + one.clone() / (one.clone() + a)
}

pub fn sol() -> i64 {
    let one = Ratio::new(1.to_biguint().unwrap(), 1.to_biguint().unwrap());
    let mut a = one + Ratio::new(1.to_biguint().unwrap(), 2.to_biguint().unwrap());
    let mut count = 0;
    for _ in 0..1000 {
        a = conv(a);
        let numer = a.numer();
        let denom = a.denom();
        if count_digits(numer.clone()) > count_digits(denom.clone()) {
            count += 1;
        }
    }
    count
}

#[test]
fn test() {
    assert_eq!(sol(), 153);
}
