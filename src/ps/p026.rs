use std::collections::HashSet;

fn qs(n: i64) -> (i64, Vec<i64>, usize) {
    let mut r: i64 = 1;
    let mut r_acc: HashSet<i64> = HashSet::new();
    let mut q_acc: Vec<i64> = Vec::new();

    loop {
        if r == 0 {
            return (n, q_acc, 0);
        } else if r_acc.contains(&r) {
            return (n, q_acc, r_acc.len());
        } else {
            r_acc.insert(r);
            q_acc.push(r / n);
            r = (r % n) * 10;
            continue;
        }
    }
}

#[rustfmt::skip]
pub fn sol() -> i64 {
    (1..=1000)
        .map(qs)
        .max_by_key(|&(_, _, cnt)| cnt)
        .unwrap()
        .0
}

#[test]
fn test() {
    assert_eq!(sol(), 983);
}
