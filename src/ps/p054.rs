use itertools::Itertools;
use lazy_static::lazy_static;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    vec,
};

lazy_static! {
    static ref STRAIGHT_SET: HashSet<String> = straight_set();
    static ref RANKS: Vec<Vec<i32>> = vec![
        vec![1, 1, 1, 1, 1], // high card
        vec![2, 1, 1, 1],    // one pair
        vec![2, 2, 1],       // two pair
        vec![3, 1, 1],       // three of a kind
        vec![],              // straight
        vec![],              // flush
        vec![3, 2],          // full house
        vec![4, 1],          // four of a kind
    ];
}

fn num_to_point(num: char) -> u32 {
    match num {
        '2'..='9' => num.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card number: {num}"),
    }
}

fn straight_set() -> HashSet<String> {
    let temp: Vec<char> = "A23456789TJQKA".chars().collect();
    temp.windows(5)
        .map(|s| s.iter().sorted().collect::<String>())
        .collect()
}

fn is_straight(hand: &[String]) -> bool {
    let card_nums: String = hand
        .iter()
        .map(|c| c.chars().next().unwrap())
        .sorted()
        .collect();
    STRAIGHT_SET.contains(&card_nums)
}

fn is_flush(hand: &[String]) -> bool {
    hand.iter()
        .map(|c| c.chars().nth(1).unwrap())
        .collect::<HashSet<char>>()
        .len() == 1
}

fn rank(hand: &[String]) -> (usize, Vec<u32>) {
    let chunks = hand
        .iter()
        .map(|c| c.chars().next().unwrap())
        .map(num_to_point)
        .sorted_by(|a, b| b.cmp(a))
        .chunk_by(|&p| p);

    let mut card_nums = Vec::new();
    for (_k, chunk) in &chunks {
        card_nums.push(chunk.collect::<Vec<_>>());
    }

    let r1 = RANKS
        .iter()
        .position(|r| {
            *r == card_nums
                .iter()
                .map(|c| c.len() as i32)
                .sorted_by(|a, b| b.cmp(a))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let r2 = card_nums
        .iter()
        .sorted_by_key(|c| -(c.len() as i32))
        .map(|c| c[0])
        .collect::<Vec<_>>();

    if is_flush(hand) {
        if r2 == vec![14, 13, 12, 11, 10] {
            (9, r2)
        } else if is_straight(hand) {
            (8, r2)
        } else {
            (5, r2)
        }
    } else if is_straight(hand) {
        (4, r2)
    } else {
        (r1, r2)
    }
}

fn load_games() -> io::Result<Vec<(Vec<String>, Vec<String>)>> {
    let mut games = Vec::new();
    let file = File::open("./data/0054_poker.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let nums: Vec<String> = line?.split(' ').map(|s| s.to_string()).collect();
        let (p1_cards, p2_cards) = nums.split_at(5);
        games.push((
            p1_cards.to_vec(),
            p2_cards.to_vec(),
        ));
    }

    Ok(games)
}

pub fn sol() -> i64 {
    let games = load_games().expect("Failed to load games");

    games
        .iter()
        .filter(|(p1, p2)| rank(p1) > rank(p2))
        .count() as i64
}

#[test]
fn test() {
    assert_eq!(sol(), 376);
}
