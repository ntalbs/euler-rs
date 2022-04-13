use itertools::iproduct;
use num::Rational64;

fn digits(n: i64) -> (i64, i64) {
    assert!((10..100).contains(&n));
    (n / 10, n % 10)
}

fn strange_cancel(n: i64, d: i64) -> Option<Rational64> {
    let (n1, n2) = digits(n);
    let (d1, d2) = digits(d);

    if n1 == d1 {
        Some(Rational64::new(n2, d2))
    } else if n1 == d2 {
        Some(Rational64::new(n2, d1))
    } else if n2 == d1 {
        Some(Rational64::new(n1, d2))
    } else if n2 == d2 {
        Some(Rational64::new(n1, d1))
    } else {
        None
    }
}

pub fn sol() -> i64 {
    *(iproduct!(10..100, 10..100)
        .filter(|(n, d)| n < d)
        .filter(|(n, d)| n % 10 != 0 && d % 10 != 0)
        .filter_map(|(n, d)| match strange_cancel(n, d) {
            Some(r1) => {
                let r0 = Rational64::new(n, d);
                if r0 == r1 {
                    Some(r1)
                } else {
                    None
                }
            }
            None => None,
        })
        .inspect(|r| println!("{:?}", r))
        .product::<Rational64>()
        .denom())
}

#[test]
fn test() {
    assert_eq!(sol(), 100);
}
