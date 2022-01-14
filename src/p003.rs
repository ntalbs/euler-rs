use crate::util::factorize;

pub fn sol() -> u64 {
    let input = 600851475143_u64;
    println!("{:?}", factorize(input));
    *factorize(input).keys().max().unwrap()
}

#[test]
fn test() {
    assert_eq!(6857, sol());
}
