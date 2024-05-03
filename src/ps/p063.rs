pub fn sol() -> i64 {
    let limit = (1.0 / (1.0 - 9.0_f64.log10())).floor() as i32;
    let mut count = 0;

    for n in 1..=limit {
        for x in 1..=9 {
            let n = f64::from(n);
            let x = f64::from(x);
            if (n - 1.0) / n <= x.log10() {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn test() {
    assert_eq!(sol(), 49);
}
