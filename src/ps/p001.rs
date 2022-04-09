#![allow(dead_code)]
#![allow(unused_variables)]

const LIMIT: i64 = 1000;

fn sol_1() -> i64 {
    (1..LIMIT)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

fn sol_2() -> i64 {
    fn sum(n: i64) -> i64 {
        n * (n + 1) / 2
    }
    fn sum_of_mul(n: i64, m: i64) -> i64 {
        m * sum(n / m)
    }

    let n = LIMIT - 1;
    sum_of_mul(n, 3) + sum_of_mul(n, 5) - sum_of_mul(n, 15)
}

pub fn sol() -> i64 {
    sol_2()
}

#[test]
fn test() {
    assert_eq!(sol_1(), 233168);
    assert_eq!(sol_2(), 233168);
}
