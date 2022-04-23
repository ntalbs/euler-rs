use crate::util::is_pentagonal;

fn p(n: i64) -> i64 {
    (n * (3 * n - 1)) / 2
}

pub fn sol() -> i64 {
    for i in 1.. {
        for j in 1..i {
            let pi = p(i);
            let pj = p(j);
            if is_pentagonal(pi + pj) && is_pentagonal(pi - pj) {
                return pi - pj;
            }
        }
    }
    -1
}

#[test]
fn test() {
    assert_eq!(sol(), 5482660);
}
