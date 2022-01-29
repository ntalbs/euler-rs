fn collatz(mut n: u64) -> u64 {
    let mut cnt: u64 = 1;
    loop {
        n = match n % 2 {
            0 => n / 2,
            1 => 3 * n + 1,
            _ => panic!("shouldn't reach here"),
        };
        if n == 1 {
            break;
        }
        cnt += 1;
    }
    cnt
}

pub fn sol() -> u64 {
    (1..1_000_000)
        .map(|n| (n, collatz(n)))
        // .inspect(|&(n, c)| println!("n={}, c={}", n, c))
        .max_by_key(|&(_n, c)| c)
        .unwrap()
        .0
}

#[test]
fn test() {
    assert_eq!(837799, sol());
}
