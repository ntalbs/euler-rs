use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use super::p018::find_maxs;

fn load_triangle() -> io::Result<Vec<Vec<i64>>> {
    let mut triangle = Vec::new();
    let file = File::open("./data/0067_triangle.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i64> = line.split(' ').map(|s| s.parse().unwrap()).collect();
        triangle.push(nums);
    }
    Ok(triangle)
}

pub fn sol() -> i64 {
    let mut triangle = load_triangle().unwrap();
    triangle.reverse();
    triangle[1..]
        .iter()
        .fold(triangle[0].clone(), |ls, us| find_maxs(&ls, us))[0]
}

#[test]
fn test() {
    assert_eq!(sol(), 7273);
}
