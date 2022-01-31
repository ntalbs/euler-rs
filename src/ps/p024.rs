use itertools::Itertools;

fn from_digits(digits: &[&u64]) -> u64 {
    let mut ret = 0;
    for x in digits {
        ret = ret * 10 + *x;
    }
    ret
}

pub fn sol() -> u64 {
    let digits: [u64;10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];


    from_digits(&digits.iter().permutations(10).nth(1_000_000 - 1).unwrap())
}

#[test]
fn test() {
    assert_eq!(sol(), 2783915460);
}
