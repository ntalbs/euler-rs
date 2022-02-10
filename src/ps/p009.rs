/// brute-force approach
fn sol_1() -> i64 {
    for a in 1..1000 {
        for b in a..1000 {
            if a + b > 1000 {
                continue;
            }
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    0
}

pub fn sol() -> i64 {
    sol_1()
}

#[test]
fn test() {
    assert_eq!(sol(), 31875000);
}
