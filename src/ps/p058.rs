use crate::util::is_prime;

fn d(n: i64) -> Vec<i64> {
    fn square(x: i64) -> i64 {
        x * x
    }
    fn d(n: i64, k: i64) -> i64 {
        2 * n * (k + 1) + square(2 * n - 1)
    }
    // d(n, k) will never be prime when k=3, hence exclude the case:
    (0..3).map(|k| d(n, k)).collect()
}

fn count_primes(xs: &[i64]) -> i64 {
    xs.iter().filter(|x| is_prime(**x)).count() as i64
}

pub fn sol() -> i64 {
    let mut n = 1;
    let mut cnt_diagonal = 1;
    let mut cnt_prime = 0;
    loop {
        let nums = d(n);
        cnt_diagonal += 4;
        cnt_prime += count_primes(&nums);

        if cnt_prime * 100 / cnt_diagonal < 10 {
            return 2 * n + 1; // side length of the square
        }
        n += 1;
    }
}

#[test]
fn test() {
    assert_eq!(sol(), 26241);
}
