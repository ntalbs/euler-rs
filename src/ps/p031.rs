fn value(n: i64) -> i64 {
    match n {
        1 => 1,
        2 => 2,
        3 => 5,
        4 => 10,
        5 => 20,
        6 => 50,
        7 => 100,
        8 => 200,
        _ => panic!("invalid index"),
    }
}

fn cc(amt: i64, n: i64) -> i64 {
    if amt == 0 {
        1
    } else if amt < 0 {
        0
    } else if n == 0 {
        0
    } else {
        cc(amt, n - 1) + cc(amt - value(n), n)
    }
}

pub fn sol() -> i64 {
    cc(200, 8)
}

#[test]
fn test() {
    assert_eq!(sol(), 73682);
}
