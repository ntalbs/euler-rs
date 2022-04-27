use crate::util::is_pentagonal;

fn h(n: i64) -> i64 {
    n * (2 * n - 1)
}

pub fn sol() -> i64 {
    (144..).map(h).find(|x| is_pentagonal(*x)).unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 1533776805);
}
