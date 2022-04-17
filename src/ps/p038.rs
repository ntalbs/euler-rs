use itertools::Itertools;

fn is_pandigital(num_str: &str) -> bool {
    "123456789" == num_str.chars().sorted().collect::<String>()
}

fn prod_concat(n: i64) -> String {
    let mut i = 1;
    let mut acc = "".to_string();
    loop {
        if acc.len() == 9 {
            break;
        }
        acc = format!("{}{}", acc, n * i);
        i += 1;
    }
    acc
}

pub fn sol() -> i64 {
    (1..9999)
        .rev()
        .map(prod_concat)
        .find(|n| is_pandigital(n))
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 932718654);
}
