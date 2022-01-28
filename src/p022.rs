use std::fs;

fn read_names() -> Vec<String> {
    let contents = fs::read_to_string("./data/names.txt").unwrap();
    contents
        .split(',')
        .map(|s| s.replace("\"", ""))
        .collect::<Vec<String>>()
}

fn score(name: &String) -> u64 {
    name.chars().map(|c| c as u64 - 64).sum()
}

pub fn sol() -> u64 {
    let mut names = read_names();

    names.sort();

    names.iter().enumerate()
        .map(|(i, name)| (i as u64 + 1) * score(name))
        .sum()
}

#[test]
fn test() {
    assert_eq!(sol(), 871198282);
}
