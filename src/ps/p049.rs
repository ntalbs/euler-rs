use crate::util::{digits, from_digits, Sieve};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{hash_map::Entry, HashMap, HashSet};

lazy_static! {
    static ref FOUR_DIGIT_PRIMES_VEC: Vec<i64> = four_digit_primes();
    static ref FOUR_DIGIT_PRIMES_SET: HashSet<i64> =
        FOUR_DIGIT_PRIMES_VEC.iter().cloned().collect();
}

fn four_digit_primes() -> Vec<i64> {
    let sieve = Sieve::new(10_000);
    let mut primes: Vec<i64> = Vec::new();
    for i in 1000..10000 {
        if sieve.is_prime(i) {
            primes.push(i as i64);
        }
    }
    primes
}

fn permutation_primes(p: i64) -> Vec<i64> {
    digits(p)
        .iter()
        .copied()
        .permutations(4)
        .unique()
        .map(|ds| from_digits(&ds))
        .filter(|n| FOUR_DIGIT_PRIMES_SET.contains(n))
        .sorted()
        .collect_vec()
}

fn find_arithmetic_sequence(v: &[i64]) -> Option<(i64, i64, i64)> {
    let mut map: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    for i in v {
        for j in v {
            if i < j {
                let delta = j - i;
                if let Entry::Vacant(e) = map.entry(delta) {
                    e.insert(vec![(*i, *j)]);
                } else {
                    let v = map.get_mut(&delta).unwrap();
                    v.push((*i, *j));
                }
            }
        }
    }

    for (_, v) in map {
        match v[..] {
            [(a, b), (c, d)] => {
                if b == c {
                    return Some((a, b, d));
                }
            }
            _ => continue,
        }
    }
    None
}

pub fn sol() -> i64 {
    let (a, b, c) = FOUR_DIGIT_PRIMES_VEC
        .iter()
        .map(|p| permutation_primes(*p))
        .filter_map(|v| find_arithmetic_sequence(&v))
        .unique()
        .nth(1)
        .unwrap();

    a * 100000000 + b * 10000 + c
}

#[test]
fn test() {
    assert_eq!(sol(), 296962999629);
}
