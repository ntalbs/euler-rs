use crate::util::lcm;

#[rustfmt::skip]
pub fn sol() -> i64 {
    (1_i64..=20)
        .fold(1, |a, x| lcm(a, x))
}

#[test]
fn test() {
    assert_eq!(sol(), 232792560);
}
