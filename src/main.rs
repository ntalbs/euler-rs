use std::env;
use std::process;
use std::time::Instant;

mod ps;
mod util;

fn time(f: fn() -> i64) {
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
        1 => ps::p001::sol,
        2 => ps::p002::sol,
        3 => ps::p003::sol,
        4 => ps::p004::sol,
        5 => ps::p005::sol,
        6 => ps::p006::sol,
        7 => ps::p007::sol,
        8 => ps::p008::sol,
        9 => ps::p009::sol,
        10 => ps::p010::sol,
        11 => ps::p011::sol,
        12 => ps::p012::sol,
        13 => ps::p013::sol,
        14 => ps::p014::sol,
        15 => ps::p015::sol,
        16 => ps::p016::sol,
        17 => ps::p017::sol,
        18 => ps::p018::sol,
        19 => ps::p019::sol,
        20 => ps::p020::sol,
        21 => ps::p021::sol,
        22 => ps::p022::sol,
        23 => ps::p023::sol,
        24 => ps::p024::sol,
        25 => ps::p025::sol,
        26 => ps::p026::sol,
        27 => ps::p027::sol,
        28 => ps::p028::sol,
        29 => ps::p029::sol,
        30 => ps::p030::sol,
        31 => ps::p031::sol,
        32 => ps::p032::sol,
        33 => ps::p033::sol,
        34 => ps::p034::sol,
        35 => ps::p035::sol,
        36 => ps::p036::sol,
        37 => ps::p037::sol,
        38 => ps::p038::sol,
        _ => {
            eprintln!("Solution #{} not implemented yet", prob_no);
            process::exit(1);
        }
    };

    time(sol);
}
