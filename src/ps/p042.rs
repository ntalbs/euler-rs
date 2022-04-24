use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::HashSet, fs};

lazy_static! {
    static ref TRIANGLE_NUMBERS: HashSet<i64> = triangle_nums(100);
}

fn triangle_nums(limit: usize) -> HashSet<i64> {
    let mut triangle_nums: HashSet<i64> = HashSet::with_capacity(limit);
    for i in 1..limit {
        let n = i as i64;
        let t = n * (n + 1) / 2;
        triangle_nums.insert(t);
    }
    triangle_nums
}

fn word_value(w: &str) -> i64 {
    w.chars().map(|c| c as i64 - 64).sum()
}

fn is_triangle_number(n: &i64) -> bool {
    TRIANGLE_NUMBERS.contains(n)
}

fn read_words() -> Vec<String> {
    fs::read_to_string("./data/words.txt")
        .unwrap()
        .split(',')
        .map(|s| s.replace('\"', ""))
        .collect_vec()
}

pub fn sol() -> i64 {
    read_words()
        .iter()
        .map(|w| word_value(w.as_str()))
        .filter(is_triangle_number)
        .count() as i64
}

#[test]
fn test() {
    assert_eq!(sol(), 162);
}
