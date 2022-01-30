struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;

        Some(self.a)
    }
}

pub fn sol() -> u64 {
    let ret = Fibonacci::new()
        .take_while(|x| x < &4_000_000)
        .filter(|x| x % 2 == 0)
        .fold(0, |a, b| a + b);
    ret
}

#[test]
fn test() {
    assert_eq!(4613732, sol());
}
