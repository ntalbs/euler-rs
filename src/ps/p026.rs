use std::collections::HashSet;

fn qs(n: u64) -> (u64, Vec<u64>, usize) {
    let mut r: u64 = 1;
    let mut r_acc: HashSet<u64> = HashSet::new();
    let mut q_acc: Vec<u64> = Vec::new();

    loop {
        if r == 0 {
            return (n, q_acc, 0);
        } else if r_acc.contains(&r) {
            return (n, q_acc, r_acc.len());
        } else {
            r_acc.insert(r);
            q_acc.push(r/n);
            r = (r % n) * 10;
            continue;
        }
    }
}

pub fn sol() -> u64 {
    (1..=1000)
        .map(|n| qs(n))
        .max_by_key(|&(_, _, cnt)| cnt)
        .unwrap()
        .0
}

#[test]
fn test() {
    assert_eq!(sol(), 983);
}