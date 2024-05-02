use std::collections::HashMap;

use crate::util::digits;

fn sorted_digits(n: i64) -> Vec<i64> {
    let mut ds = digits(n);
    ds.sort();
    ds
}

pub fn sol() -> i64 {
    let mut i: i64 = 1;
    let mut m: HashMap<Vec<i64>, Vec<i64>> = HashMap::new();
    loop {
        let c = i.pow(3);
        let k = sorted_digits(c);

        #[warn(clippy::unwrap_or_default)]
        m.entry(k.clone()).or_insert(vec![]).push(c);

        if m[&k].len() == 5 {
            return *m[&k].iter().min().unwrap();
        }
        i += 1;
    }
}

#[test]
fn test() {
    assert_eq!(sol(), 127035954683);
}
