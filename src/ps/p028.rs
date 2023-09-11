use itertools::iproduct;

fn square(n: i64) -> i64 {
    n * n
}

fn d(n: i64, k: i64) -> i64 {
    2 * n * (k + 1) + square(2 * n - 1)
}

pub fn sol() -> i64 {
    const SIZE: i64 = 1001;
    iproduct!(1..=(SIZE / 2), 0..4)
        .map(|(n, k)| d(n, k))
        .sum::<i64>()
        + 1
}

#[test]
fn test() {
    assert_eq!(sol(), 669171001);
}
