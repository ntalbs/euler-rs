use crate::util::factorize;

pub fn sol() -> i64 {
    let input = 600851475143_i64;
    println!("{:?}", factorize(input));
    *factorize(input).keys().max().unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 6857);
}
