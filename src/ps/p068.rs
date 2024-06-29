use itertools::{self, Itertools};

fn conv(perm: &Vec<&i32>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    let a = *perm[0];
    let b = *perm[1];
    let c = *perm[2];
    let d = *perm[3];
    let e = *perm[4];
    let f = *perm[5];
    let g = *perm[6];
    let h = *perm[7];
    let i = *perm[8];
    let j = *perm[9];
    (a, b, c, d, e, f, g, h, i, j)
}

pub fn sol() -> i64 {
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .permutations(10)
        .filter(|perm| {
            let (a, b, c, d, e, f, g, h, i, j) = conv(perm);
            let sum = a + b + c;
            d + c + e == sum && f + e + g == sum && h + g + i == sum && j + i + b == sum
        })
        .filter(|perm| {
            let (a, _, _, d, _, f, _, h, _, j) = conv(perm);
            a == *[a, d, f, h, j].iter().min().unwrap()
        })
        .map(|perm| {
            let (a, b, c, d, e, f, g, h, i, j) = conv(&perm);
            format!("{a}{b}{c}{d}{c}{e}{f}{e}{g}{h}{g}{i}{j}{i}{b}")
        })
        .filter(|s| s.len() == 16)
        .map(|s| s.parse::<i64>().unwrap())
        .max()
        .unwrap()
}

#[test]
fn test() {
    assert_eq!(sol(), 6531031914842725);
}
