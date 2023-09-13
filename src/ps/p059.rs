use itertools::iproduct;
use std::fs;

fn encrypted_message() -> Vec<u8> {
    fs::read_to_string("./data/0059_cipher.txt")
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect()
}

fn decipher(encrypted: &[u8], key: &[u8]) -> String {
    let mut buf = Vec::new();
    let mut k = 0;
    for c in encrypted {
        buf.push(c ^ key[k]);
        k = (k + 1) % 3;
    }
    String::from_utf8(buf).unwrap()
}

pub fn sol() -> i64 {
    let encrypted = encrypted_message();
    let rng = b'a'..=b'z';

    for (a, b, c) in iproduct!(rng.clone(), rng.clone(), rng.clone()) {
        let key = [a, b, c];
        let decrypted = decipher(&encrypted, &key);

        if decrypted.contains(" the ") {
            return decrypted.as_bytes().iter().map(|x| *x as i64).sum();
        }
    }
    panic!("Didn't find answer.");
}

#[test]
fn test() {
    assert_eq!(sol(), 129448);
}
