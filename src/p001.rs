const LIMIT: u64 = 1000;

fn sol_1() -> u64 {
    (1..LIMIT)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |a, x| a + x)
}

fn sol_2() -> u64 {
    fn sum(n: u64) -> u64 {
        n * (n + 1) / 2
    }
    fn sum_of_mul(n: u64, m: u64) -> u64 {
        m * sum(n / m)
    }

    let n = LIMIT - 1;
    sum_of_mul(n, 3) + sum_of_mul(n, 5) - sum_of_mul(n, 15)
}

pub fn sol() -> u64 {
    sol_2()
}
