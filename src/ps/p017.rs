#[rustfmt::skip]
fn num2str(n: i64) -> String {
    let one_nineteen = [
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    let deca = vec![
        "",
        "ten",
        "twenty",
        "thirty",
        "forty",
        "fifty",
        "sixty",
        "seventy",
        "eighty",
        "ninety",
    ];

    let num2str_under_20 = |n: usize| -> String {
        one_nineteen[n].to_string()
    };

    let num2str_under_100 = |n: usize| -> String {
        if n < 20 {
            num2str_under_20(n)
        } else {
            let d1 = &deca[n / 10];
            let d2 = &one_nineteen[n % 10];
            format!("{}{}", d1, d2)
        }
    };

    let num2str_under_1000 = |n: usize| -> String {
        if n < 100 {
            num2str_under_100(n)
        } else {
            let d1 = &one_nineteen[n / 100];
            let dr = num2str_under_100(n % 100);
            if dr.is_empty() {
                format!("{}hundred", d1)
            } else {
                format!("{}hundredand{}", d1, dr)
            }
        }
    };

    let i = n as usize;
    match n {
        1..=999 => num2str_under_1000(i),
        1000 => format!("{}{}", "one", "thousand"),
        _ => panic!("should not reach here."),
    }
}

pub fn sol() -> i64 {
    (1..=1000)
        .map(num2str)
        .map(|s| s.chars().count())
        .sum::<usize>() as i64
}

#[test]
fn test() {
    assert_eq!(sol(), 21124);
}
