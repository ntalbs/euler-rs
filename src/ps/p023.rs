use std::collections::HashSet;

use crate::util::aliquot_sum;

const LIMIT: i64 = 28123;

fn is_abundant(n: i64) -> bool {
    aliquot_sum(n) > n
}

pub fn sol() -> i64 {
    let abundants = (1..=LIMIT)
        .filter(|n| is_abundant(*n))
        .collect::<Vec<i64>>();

    let mut sum_of_two_abundants: HashSet<i64> = HashSet::new();

    for a in &abundants {
        for b in &abundants {
            if a > b || a + b > LIMIT {
                continue;
            }
            sum_of_two_abundants.insert(a + b);
        }
    }

    (1..=LIMIT)
        .filter(|n| !sum_of_two_abundants.contains(n))
        .sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 4179871);
}
