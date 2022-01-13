use std::collections::{BTreeMap, LinkedList};

/// Returns the greatest common divisor of m and n.
pub fn gcd(mut m: u64, mut n: u64) -> u64 {
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
pub fn lcm(m: u64, n: u64) -> u64 {
    (m * n) / gcd(m, n)
}

pub fn pow(x: u64, e: u64) -> u64 {
    let mut ret = 1;
    for _ in 0..e {
        ret *= x;
    }
    ret
}

/// Returns prime factorized result of a specified number as a map of prime factor to exponent
pub fn factorize(mut n: u64) -> BTreeMap<u64, u64> {
    let mut ret = BTreeMap::new();
    for p in Primes::new() {
        if n == 1 {
            break;
        }
        if p > (n as f64).sqrt() as u64 {
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

pub fn is_palindrome(mut n: u64) -> bool {
    fn count_digits(mut n: u64) -> u64 {
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
        let pow = pow(10, num_digits - 1);
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

pub struct Primes {
    primes: LinkedList<u64>,
    next: u64,
}

impl Primes {
    pub fn new() -> Self {
        Primes {
            primes: LinkedList::new(),
            next: 2,
        }
    }
    fn is_prime(&self, n: u64) -> bool {
        let limit = (n as f64).sqrt() as u64 + 1;
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
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
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
fn test_pow() {
    assert_eq!(2 * 2 * 2, pow(2, 3));
    assert_eq!(3 * 3 * 3, pow(3, 3));
}

#[test]
fn test_factorize() {
    fn compare(input: u64, factors: BTreeMap<u64, u64>) {
        let mut comp: u64 = 1;
        for (p, e) in factors {
            comp *= pow(p, e);
        }
        assert_eq!(input, comp);
    }

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
    let primes_under_20_ref: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let primes_under_20_gen: Vec<u64> = Primes::new().take_while(|x| x < &20_u64).collect();
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
