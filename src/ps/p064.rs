fn is_square_num(n: i64) -> bool {
    let sqrt = (n as f64).sqrt() as i64;
    sqrt * sqrt == n
}

fn expand_continued_fraction(n :i64) -> Vec<i64> {
    let a0 = (n as f64).sqrt().floor() as i64;
    let mut acc = vec![a0];
    let mut m = 0;
    let mut d = 1;
    let mut a = a0;
    loop {
        if a == 2 * a0 {
            return acc;
        }
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a0 + m) / d;
        acc.push(a);
    }
}

pub fn sol() -> i64 {
    (2..=10_000)
        .filter(|&n| !is_square_num(n))
        .map(|n| expand_continued_fraction(n))
        .filter(|v| v.len() % 2 == 0)
        .count() as i64
}

#[test]
fn test() {
    assert_eq!(sol(), 1322);
}
