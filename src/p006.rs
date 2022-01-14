fn square_of_sum(n: u64) -> u64 {
    let sum: u64 = (1..=n).sum();
    sum * sum
}

fn sum_of_square(n: u64) -> u64 {
    (1..=n).map(|x| x * x).sum()
}

pub fn sol() -> u64 {
    const LIMIT: u64 = 100;
    square_of_sum(LIMIT) - sum_of_square(LIMIT)
}

#[test]
fn test() {
    assert_eq!(25164150, sol());
}
