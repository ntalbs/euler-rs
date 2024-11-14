use std::collections::HashMap;

use itertools::Itertools;

fn p(k: i64, n: i64) -> i64 {
    match k {
        3 => n * (n + 1) / 2,
        4 => n * n,
        5 => (n * ((3 * n) - 1)) / 2,
        6 => n * ((2 * n) - 1),
        7 => (n * ((5 * n) - 3)) / 2,
        8 => n * ((3 * n) - 2),
        _ => panic!("k should be [3, 8] but was {k}"),
    }
}

fn s(k: i64) -> Vec<i64> {
    (1..)
        .map(|n| p(k, n))
        .skip_while(|x| *x < 1_000)
        .take_while(|x| *x < 10_000)
        .collect()
}

fn ps(k: i64) -> Vec<(i64, i64)> {
    s(k).iter().map(|&n| (k, n)).collect_vec()
}

fn first_2_digits(n: i64) -> i64 {
    n / 100
}

fn last_2_digits(n: i64) -> i64 {
    n % 100
}

fn chain(m: &HashMap<i64, Vec<(i64, i64)>>, ps: Vec<Vec<(i64, i64)>>) -> Vec<Vec<(i64, i64)>> {
    for e in ps {
        let chain = match m.get(&last_2_digits(e.last().unwrap().1)) {
            Some(chain) => chain,
            None => continue,
        };
        for _t in chain {}
    }

    vec![]
}

pub fn sol() -> i64 {
    let m: HashMap<i64, Vec<(i64, i64)>> = (3..9)
        .flat_map(ps)
        .sorted_by_key(|(_, n)| *n)
        .chunk_by(|(_, n)| first_2_digits(*n))
        .into_iter()
        .map(|(k, g)| (k, g.collect::<Vec<(i64, i64)>>()))
        .collect();

    println!("{m:?}");
    0
}

#[test]
fn test() {
    assert_eq!(sol(), 28684);
}

#[test]
fn test_1() {
    println!("3=>{:?}", s(3));
    println!("4=>{:?}", s(4));
    println!("5=>{:?}", s(5));
    println!("6=>{:?}", s(6));
    println!("7=>{:?}", s(7));
    println!("8=>{:?}", s(8));
}
