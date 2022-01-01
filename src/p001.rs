const LIMIT: i32 = 1000;

pub fn sol_1() -> i32 {
    (1..LIMIT)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |a, x| a + x)
}

pub fn sol_2() -> i32 {
    fn sum(n: i32) -> i32 {
        n * (n + 1) / 2
    }
    fn sum_of_mul(n: i32, m: i32) -> i32 {
        m * sum(n / m)
    }

    let n = LIMIT - 1;
    sum_of_mul(n, 3) + sum_of_mul(n, 5) - sum_of_mul(n, 15)
}
