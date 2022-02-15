fn factorial(n: i64) -> i64 {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        7 => 5040,
        8 => 40320,
        9 => 362880,
        _ => panic!()
    }
}

fn sum_of_fact(mut n: i64) -> i64{
    let mut ret = 0;
    loop {
        if n == 0 {
            break;
        }
        ret += factorial(n % 10);
        n /= 10;
    }
    ret
}

fn check(n: i64) -> bool {
    n == sum_of_fact(n)
}

pub fn sol() -> i64 {
    (11..=2_540_160)
        .filter(|n| check(*n))
        .sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 40730)
}
