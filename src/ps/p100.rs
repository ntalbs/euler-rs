pub fn sol() -> i64 {
    const TARGET: i64 = 1_000_000_000_000;
    let next_b = |b: i64, n: i64| -> i64 { 3 * b + 2 * n - 2 };
    let next_n = |b: i64, n: i64| -> i64 { 4 * b + 3 * n - 3 };

    let mut b: i64 = 15;
    let mut n: i64 = 21;
    loop {
        if n >= TARGET {
            return b;
        }
        (b, n) = (next_b(b, n), next_n(b, n));
    }
}

#[test]
fn test() {
    assert_eq!(sol(), 756872327473);
}
