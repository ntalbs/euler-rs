use crate::util::digits;

fn sum_of_5th_power_of_digits(n: i64) -> i64 {
    digits::<i64, i64>(n)
        .iter()
        .map(|d| d * d * d * d * d)
        .sum()
}

pub fn sol() -> i64 {
    (2..354294)
        .filter(|n| sum_of_5th_power_of_digits(*n) == *n)
        .sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 443839);
}
