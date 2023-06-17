use crate::util::{digits, from_digits};

fn sorted(x: i64) -> i64 {
    let mut ds: Vec<i64> = digits(x);
    ds.sort();
    from_digits(&ds)
}

fn are_all_same_digits(v: [i64; 6]) -> bool {
    let base = sorted(v[0]);
    base == sorted(v[1])
        && base == sorted(v[2])
        && base == sorted(v[3])
        && base == sorted(v[4])
        && base == sorted(v[5])
}

pub fn sol() -> i64 {
    (1..)
        .map(|x| [x * 1, x * 2, x * 3, x * 4, x * 5, x * 6])
        .filter(|xs| are_all_same_digits(*xs))
        .next()
        .unwrap()[0]
}

#[test]
fn test() {
    assert_eq!(sol(), 142857);
}
