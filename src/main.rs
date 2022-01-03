use std::env;
use std::process;
use std::time::Instant;

mod p001;
mod p002;
mod util;

fn time(f: fn() -> u64) {
    let start = Instant::now();
    let ret = f();
    println!("took {} ms: {}", start.elapsed().as_millis(), ret);
}

fn show_usage() {
    println!("Usage: euler-rs [num]\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        show_usage();
        eprintln!("Please, specify problem number.");
        process::exit(1);
    }

    let prob_no = match args[1].parse::<u32>() {
        Ok(n) => n,
        Err(e) => {
            show_usage();
            eprintln!("Error: problem number is not valid.");
            process::exit(1);
        }
    };

    let sol = match prob_no {
        1 => p001::sol,
        2 => p002::sol,
        _ => panic!("not implemented yet"),
    };

    time(sol);
}
