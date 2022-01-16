use std::env;
use std::process;
use std::time::Instant;

mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;
mod p007;
mod p008;
mod p009;
mod p010;
mod p011;
mod p012;
mod p013;
mod util;

fn time(f: fn() -> u64) {
    let start = Instant::now();
    let ret = f();
    println!("Elapsed time: {} ms", start.elapsed().as_millis());
    println!("Answer: {}", ret);
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
        Err(_) => {
            show_usage();
            eprintln!("Error: problem number is not valid.");
            process::exit(1);
        }
    };

    let sol = match prob_no {
        1 => p001::sol,
        2 => p002::sol,
        3 => p003::sol,
        4 => p004::sol,
        5 => p005::sol,
        6 => p006::sol,
        7 => p007::sol,
        8 => p008::sol,
        9 => p009::sol,
        10 => p010::sol,
        11 => p011::sol,
        12 => p012::sol,
        13 => p013::sol,
        _ => {
            eprintln!("Solution #{} not implemented yet", prob_no);
            process::exit(1);
        }
    };

    time(sol);
}
