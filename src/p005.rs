use crate::util::lcm;

#[rustfmt::skip]
pub fn sol() -> u64 {
    (1_u64..=20)
        .fold(1, |a, x| lcm(a, x))
}

#[test]
fn test() {
    assert_eq!(232792560, sol());
}
