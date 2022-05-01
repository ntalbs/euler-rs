use crate::util::factorize;
use itertools::Itertools;

#[rustfmt::skip]
pub fn sol() -> i64 {
    (210..)
        .map(|n| (n, factorize(n).len()))
        .tuple_windows::<(_, _, _, _)>()
        .find(|w| w.0 .1 == 4 && w.1 .1 == 4 && w.2 .1 == 4 && w.3 .1 == 4)
        .unwrap()
        .0
        .0
}

#[test]
fn test() {
    assert_eq!(sol(), 134043);
}
