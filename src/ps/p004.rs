use crate::util::is_palindrome;

pub fn sol() -> i64 {
    let mut products: Vec<i64> = Vec::with_capacity(500_000);
    for a in 100..=999 {
        for b in a..=999 {
            products.push(a * b);
        }
    }
    *products
        .iter()
        .filter(|x| is_palindrome(**x))
        .max()
        .unwrap()
}

#[test]
fn test() {
    assert_eq!(906609, sol());
}
