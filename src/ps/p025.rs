use crate::util::Fibonacci;
use num::BigUint;

pub fn sol() -> i64 {
    let n = Fibonacci::<BigUint>::new()
        .enumerate()
        .skip_while(|(_, x)| x.to_str_radix(10).len() < 1000)
        .next()
        .unwrap().0;
    (n + 1) as i64
}

#[test]
fn test() {
    assert_eq!(sol(), 4782);
}
