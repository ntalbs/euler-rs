use std::collections::HashSet;
use crate::util::digits;

pub fn sol() -> i64 {
    let indexes: HashSet<usize> = HashSet::from([1, 10, 100, 1000, 10000, 100000, 1000000]);

    (1_i64..)
        .flat_map(|n| digits::<i64, i64>(n))
        .take(1_000_000)
        .enumerate()
        .filter(|(i, _)| indexes.contains(&(i + 1)))
        .map(|(_, d)| d)
        .product()
}

#[test]
fn test() {
    assert_eq!(sol(), 210);
}
