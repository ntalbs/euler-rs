use itertools::Itertools;

use crate::util::from_digits;

pub fn sol() -> i64 {
    (0..=9)
        .permutations(10)
        .skip_while(|ds| ds[0] == 0)
        .filter(|ds| from_digits(&ds[7..10]) % 17 == 0)
        .filter(|ds| from_digits(&ds[6..9]) % 13 == 0)
        .filter(|ds| from_digits(&ds[5..8]) % 11 == 0)
        .filter(|ds| from_digits(&ds[3..6]) % 5 == 0)
        .filter(|ds| from_digits(&ds[4..7]) % 7 == 0)
        .filter(|ds| from_digits(&ds[2..5]) % 3 == 0)
        .filter(|ds| from_digits(&ds[1..4]) % 2 == 0)
        .map(|ds| from_digits(&ds))
        .sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 16695334890);
}
