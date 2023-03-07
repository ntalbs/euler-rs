use itertools::Itertools;

fn from_digits(v: &[&i64]) -> i64 {
    let mut ret = 0;
    for d in v {
        ret = ret * 10 + *d;
    }
    ret
}

fn check_pattern(v: &[&i64], n1: usize, n2: usize) -> Option<i64> {
    let a = from_digits(&v[0..n1]);
    let b = from_digits(&v[n1..(n1 + n2)]);
    let c = from_digits(&v[(n1 + n2)..]);

    if a * b == c {
        println!("{a} x {b} = {c}");
        Some(c)
    } else {
        None
    }
}

fn check(v: &[&i64]) -> Vec<Option<i64>> {
    vec![check_pattern(v, 1, 4), check_pattern(v, 2, 3)]
}

pub fn sol() -> i64 {
    let digits: [i64; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    digits
        .iter()
        .permutations(9)
        .flat_map(|v| check(&v))
        .flatten()
        .unique()
        .sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 45228);
}
