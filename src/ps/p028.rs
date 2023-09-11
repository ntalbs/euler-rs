use itertools::iproduct;

fn d(n: i64, k: i64) -> i64 {
    2 * n * (k + 1) + (2 * n - 1).pow(2)
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
