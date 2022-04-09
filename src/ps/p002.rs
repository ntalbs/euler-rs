use crate::util::Fibonacci;

pub fn sol() -> i64 {
    Fibonacci::<i64>::new()
        .take_while(|x| x < &4_000_000)
        .filter(|x| x % 2 == 0)
        .sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 4613732);
}
