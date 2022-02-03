use num::ToPrimitive;

use crate::util::factorial;

pub fn sol() -> i64 {
    let f40 = factorial(40);
    let f20 = factorial(20);

    (&f40 / &f20 / &f20).to_i64().unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 137846528820);
}
