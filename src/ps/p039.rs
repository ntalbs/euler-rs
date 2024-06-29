use itertools::{iproduct, Itertools};
use num::integer::Roots;

use crate::util::gcd;

fn triplet(m: i64, n: i64, k: i64) -> (i64, i64, i64) {
    (k * (m * m - n * n), k * 2 * m * n, k * (m * m + n * n))
}

pub fn sol() -> i64 {
    const LIMIT: i64 = 1000;
    iproduct!(
        2..((LIMIT / 4).sqrt()),
        1..((LIMIT / 4).sqrt()),
        1..(LIMIT / 12)
    )
    .filter(|(m, n, _k)| n < m && (m - n) % 2 == 1 && gcd(*m, *n) == 1)
    .map(|(m, n, k)| triplet(m, n, k))
    .filter(|(m, n, k)| m + n + k <= LIMIT)
    .map(|(m, n, k)| m + n + k)
    .sorted()
    .chunk_by(|p| *p)
    .into_iter()
    .map(|(key, group)| (key, group.count()))
    .max_by_key(|&(_, count)| count)
    .unwrap()
    .0
}

#[test]
fn test() {
    assert_eq!(sol(), 840);
}
