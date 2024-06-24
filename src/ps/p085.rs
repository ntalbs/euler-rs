use itertools::iproduct;

fn count_rect(m: i64, n: i64) -> i64 {
    (m * (m + 1) * n * (n + 1)) / 4
}

fn delta(n: i64) -> i64 {
    (2_000_000 - n).abs()
}

const MAX: i64 = 100;
pub fn sol() -> i64 {
    let mut data: Vec<(i64, i64, i64)> = iproduct!(1..MAX, 1..MAX)
        .map(|(m, n)| {
            let count = count_rect(m, n);
            (m, n, delta(count))
        })
        .collect();

    data.sort_by_key(|&(_, _, delta)| delta);
    let (m, n, _) = data[0];
    m * n
}

#[test]
fn test() {
    assert_eq!(sol(), 2772);
}
