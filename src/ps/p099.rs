use std::fs::read_to_string;

use itertools::Itertools;

pub fn sol() -> i64 {
    (read_to_string("./data/base_exp.txt").unwrap().lines()
        .map(|line| line.split(',').next_tuple().unwrap())
        .map(|(b, e)| (b.parse::<f64>().unwrap(), e.parse::<f64>().unwrap()))
        .map(|(b, e)| e * b.log(10.0))
        .enumerate()
        .max_by_key(|(_, x)| *x as u64)
        .unwrap().0 + 1) as i64
}

#[test]
fn test() {
    assert_eq!(sol(), 709);
}
