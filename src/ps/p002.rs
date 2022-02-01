use crate::util::Fibonacci;

pub fn sol() -> u64 {
    let ret = Fibonacci::<u64>::new()
        .take_while(|x| x < &4_000_000)
        .filter(|x| x % 2 == 0)
        .fold(0, |a, b| a + b);
    ret
}

#[test]
fn test() {
    assert_eq!(4613732, sol());
}
