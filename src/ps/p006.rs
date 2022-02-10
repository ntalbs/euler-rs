fn square_of_sum(n: i64) -> i64 {
    let sum: i64 = (1..=n).sum();
    sum * sum
}

fn sum_of_square(n: i64) -> i64 {
    (1..=n).map(|x| x * x).sum()
}

pub fn sol() -> i64 {
    const LIMIT: i64 = 100;
    square_of_sum(LIMIT) - sum_of_square(LIMIT)
}

#[test]
fn test() {
    assert_eq!(sol(), 25164150);
}
