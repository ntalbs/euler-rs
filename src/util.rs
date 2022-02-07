use std::{collections::{BTreeMap, LinkedList}, ops::Add};
use num::{bigint::ToBigUint, BigUint, Num, pow};

/// Returns the greatest common divisor of m and n.
pub fn gcd(mut m: i64, mut n: i64) -> i64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

/// Returns the least common multiplier of m and n.
pub fn lcm(m: i64, n: i64) -> i64 {
    (m * n) / gcd(m, n)
}

/// Returns prime factorized result of a specified number as a map of prime factor to exponent
pub fn factorize(mut n: i64) -> BTreeMap<i64, i64> {
    let mut ret = BTreeMap::new();
    for p in Primes::new() {
        if n == 1 {
            break;
        }
        if p > (n as f64).sqrt() as i64 {
            ret.insert(n, 1);
            break;
        }
        while n % p == 0 {
            let e = ret.entry(p).or_insert(0);
            *e += 1;
            n /= p;
        }
    }
    ret
}

pub fn is_palindrome(mut n: i64) -> bool {
    fn count_digits(mut n: i64) -> i64 {
        let mut cnt = 1;
        loop {
            n /= 10;
            if n == 0 {
                break;
            }
            cnt += 1;
        }
        cnt
    }

    let mut num_digits = count_digits(n);

    while num_digits > 1 {
        let pow = pow(10, num_digits as usize - 1);
        let msd = n / pow; // most significant digit
        let lsd = n % 10; // least significant digit
        if msd != lsd {
            return false;
        }
        n = (n % pow) / 10;
        num_digits -= 2;
    }
    true
}

pub fn is_prime(n: i64) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 { return false; }
    if n < 9 { return true; }
    if n % 3 == 0 { return false; }
    for i in (11..((n as f64).sqrt() as i64)).step_by(2)  {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub struct Primes {
    primes: LinkedList<i64>,
    next: i64,
}

impl Primes {
    pub fn new() -> Self {
        Primes {
            primes: LinkedList::new(),
            next: 2,
        }
    }
    fn is_prime(&self, n: i64) -> bool {
        let limit = (n as f64).sqrt() as i64 + 1;
        for p in &self.primes {
            if n % p == 0 {
                return false;
            }
            if *p > limit {
                break;
            }
        }
        true
    }
}

impl Iterator for Primes {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        loop {
            if self.is_prime(self.next) {
                let current = self.next;
                self.primes.push_back(self.next);
                self.next += 1;
                return Some(current);
            }
            self.next += 1;
        }
    }
}

pub struct Sieve {
    primes: Vec<bool>,
    limit: usize,
}

impl Sieve {
    pub fn new(limit: usize) -> Self {
        let mut sieve = Sieve {
            primes: Vec::with_capacity(limit / 2),
            limit,
        };
        sieve.init();
        sieve
    }

    fn init(&mut self) {
        self.primes.push(false);
        for _ in 1..self.primes.capacity() {
            self.primes.push(true);
        }

        // Skips even numbers, and even multipliers
        for n in (3..self.limit).step_by(2) {
            for m in (3..).step_by(2) {
                if m * n >= self.limit {
                    break;
                }
                let i = (m * n - 1) / 2; // array index for multiples of n
                self.primes[i] = false;
            }
        }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        match n {
            0 | 1 => false, // 0, 1 is not prime
            2 => true,      // 2 is prime
            _ => {
                if n % 2 == 0 {
                    // even numbers other than 2 are not prime
                    false
                } else {
                    // for odd numbers bigger than 2, check sieve
                    self.primes[(n - 1) / 2]
                }
            }
        }
    }
}

pub struct Fibonacci<T> {
    a: T,
    b: T,
}

impl<T: Num> Fibonacci<T> {
    pub fn new() -> Self {
        Self { a: T::one(), b: T::one() }
    }
}

impl<T> Iterator for Fibonacci<T> where for<'a> &'a T: Add<&'a T, Output=T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        use std::mem::swap;
        let mut next = &self.a + &self.b;
        swap(&mut self.a, &mut next);
        swap(&mut self.a, &mut self.b);
        Some(next)
    }
}

pub fn factorial(n: i64) -> BigUint {
    (1..=n).map(|n| n.to_biguint().unwrap()).product()
}

pub fn digits<T, R>(n: T) -> Vec<R> where
    T: Num + ToString,
    R: Num + TryFrom<u32>
{
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| R::try_from(d).ok().unwrap())
        .collect()
}

pub fn aliquot_sum(n: i64) -> i64 {
    if n <= 1 {
        return 0;
    }
    factorize(n)
        .iter()
        .map(|(p, e)| (pow(*p, *e as usize + 1) - 1) / (p - 1))
        .product::<i64>()
        - n
}

#[test]
#[should_panic]
fn test_gcd_0_x() {
    gcd(0, 10);
}

#[test]
#[should_panic]
fn test_gcd_x_0() {
    gcd(10, 0);
}

#[test]
fn test_gcd() {
    assert_eq!(1, gcd(1, 10));
    assert_eq!(1, gcd(10, 1));
    assert_eq!(2, gcd(2, 10));
    assert_eq!(4, gcd(4, 12));
    assert_eq!(4, gcd(8, 12));
    assert_eq!(3, gcd(6, 9));
}

#[test]
fn test_lcm() {
    assert_eq!(2 * 3, lcm(2, 3));
    assert_eq!(3 * 5, lcm(3, 5));
    assert_eq!(15, lcm(5, 15));
}

#[test]
fn test_factorize() {
    fn compare(input: i64, factors: BTreeMap<i64, i64>) {
        let mut comp: i64 = 1;
        for (p, e) in factors {
            comp *= pow(p, e as usize);
        }
        assert_eq!(input, comp);
    }

    #[rustfmt::skip]
    let data = vec![
        2 * 3 * 5,
        12 * 24 * 48,
        2 * 2 * 101 * 1299721,
    ];

    for i in data {
        compare(i, factorize(i));
    }
}

#[test]
fn test_is_palindrome() {
    let palindromes = vec![
        //1, 11, 121, 1221,
        12321, 123321, 1234321, 12344321,
    ];
    for n in palindromes {
        println!("{} -> {}", n, is_palindrome(n));
        assert!(is_palindrome(n));
    }

    let non_palindromes = vec![10, 123, 1222, 12320, 123432, 123421, 1234322, 12344331];
    for n in non_palindromes {
        assert!(!is_palindrome(n));
    }
}

#[test]
fn test_prime_iter() {
    let primes_under_20_ref: Vec<i64> = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let primes_under_20_gen: Vec<i64> = Primes::new().take_while(|x| x < &20_i64).collect();
    assert_eq!(primes_under_20_ref, primes_under_20_gen);
}

#[test]
fn test_prime_sieve() {
    let primes_under_20_ref: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let sieve = Sieve::new(20);
    for p in primes_under_20_ref {
        assert!(sieve.is_prime(p));
    }
}

#[test]
fn test_prime_iter_with_sieve() {
    let primes_iter = Primes::new().take_while(|p| p < &1000);
    let sieve = Sieve::new(1000);

    for p in primes_iter {
        assert!(sieve.is_prime(p as usize));
    }
}

#[test]
fn test_fibonacci_i64() {
    let mut fibo = Fibonacci::<i64>::new();
    assert_eq!(fibo.next().unwrap(), 1);
    assert_eq!(fibo.next().unwrap(), 1);
    assert_eq!(fibo.next().unwrap(), 2);
    assert_eq!(fibo.next().unwrap(), 3);
    assert_eq!(fibo.next().unwrap(), 5);
    assert_eq!(fibo.next().unwrap(), 8);
    assert_eq!(fibo.next().unwrap(), 13);
    assert_eq!(fibo.next().unwrap(), 21);
    assert_eq!(fibo.next().unwrap(), 34);
}

#[test]
fn test_fibonacci_biguint() {
    let mut fibo = Fibonacci::<BigUint>::new();
    assert_eq!(fibo.next().unwrap(), 1_i32.to_biguint().unwrap());
    assert_eq!(fibo.next().unwrap(), 1_i32.to_biguint().unwrap());
    assert_eq!(fibo.next().unwrap(), 2_i32.to_biguint().unwrap());
    assert_eq!(fibo.next().unwrap(), 3_i32.to_biguint().unwrap());
    assert_eq!(fibo.next().unwrap(), 5_i32.to_biguint().unwrap());
    assert_eq!(fibo.next().unwrap(), 8_i32.to_biguint().unwrap());
    assert_eq!(fibo.next().unwrap(), 13_i32.to_biguint().unwrap());
    assert_eq!(fibo.next().unwrap(), 21_i32.to_biguint().unwrap());
    assert_eq!(fibo.next().unwrap(), 34_i32.to_biguint().unwrap());
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1.to_biguint().unwrap());
    assert_eq!(factorial(1), 1.to_biguint().unwrap());
    assert_eq!(factorial(2), 2.to_biguint().unwrap());
    assert_eq!(factorial(3), (1 * 2 * 3).to_biguint().unwrap());
    assert_eq!(factorial(4), (1 * 2 * 3 * 4).to_biguint().unwrap());
    assert_eq!(factorial(5), (1 * 2 * 3 * 4 * 5).to_biguint().unwrap());
}

#[test]
fn test_digits() {
    assert_eq!(digits::<u64, u8>(12345_u64), vec![1, 2, 3, 4, 5]);
    assert_eq!(digits::<i32, i8>(54321_i32), vec![5, 4, 3, 2, 1]);
}
