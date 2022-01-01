use std::time::Instant;

mod p001;

fn time(f: &dyn Fn()->i32) {
    let start = Instant::now();
    let ret = f();
    println!("took {} ms: {}", start.elapsed().as_millis(), ret);
}
fn main() {
    time(&p001::sol_1);
    time(&p001::sol_2);
}
