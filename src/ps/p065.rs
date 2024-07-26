use num::{BigInt, BigRational};

use crate::util::digits;

fn xs(n: usize) -> Vec<i64> {
    let mut vec: Vec<i64> = (1..)
        .flat_map(|i| [1, 2 * i, 1])
        .take(n - 1)
        .collect();
    vec.reverse();
    vec.push(2);
    vec
}

fn f(a: BigRational, b: BigRational) -> BigRational {
    b + a.recip()
}

pub fn sol() -> i64 {
    let xs = xs(100);
    let init = BigRational::from(BigInt::from(xs[0]));
    let binding = xs[1..]
        .iter()
        .map(|&n| BigRational::from(BigInt::from(n)))
        .fold(init, f);
    let nu = binding.numer();
    digits(nu.clone()).iter().sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 272);
}
