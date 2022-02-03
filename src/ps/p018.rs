use std::cmp::max;

fn find_maxs(ls: &[i64], us: &[i64]) -> Vec<i64> {
    let sum1 = ls.iter().zip(us.iter()).map(|(a, b)| a + b);
    let sum2 = ls.iter().skip(1).zip(us.iter()).map(|(a, b)| a + b);
    sum1.zip(sum2).map(|(a, b)| max(a, b)).collect()
}

pub fn sol() -> i64 {
    let mut triangle: Vec<Vec<i64>> = vec![
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 4, 82, 47, 65],
        vec![19, 1, 23, 75, 3, 34],
        vec![88, 2, 77, 73, 7, 63, 67],
        vec![99, 65, 4, 28, 6, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
    ];

    triangle.reverse();
    triangle[1..]
        .into_iter()
        .fold(triangle[0].clone(), |ls, us| find_maxs(&ls, &us))[0]
}

#[test]
fn test_find_max() {
    assert_eq!(find_maxs(&vec![2, 3], &vec![1]), [4]);
    assert_eq!(find_maxs(&vec![2, 3, 4], &vec![1, 2]), [4, 6]);
}

#[test]
fn test() {
    assert_eq!(sol(), 1074);
}
