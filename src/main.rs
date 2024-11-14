use std::env;
use std::process;
use std::time::Instant;

mod ps;
mod util;

fn time(f: fn() -> i64) {
    let start = Instant::now();
    let ret = f();
    println!("Elapsed time: {} ms", start.elapsed().as_millis());
    println!("Answer: {ret}");
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
        39 => ps::p039::sol,
        40 => ps::p040::sol,
        41 => ps::p041::sol,
        42 => ps::p042::sol,
        43 => ps::p043::sol,
        44 => ps::p044::sol,
        45 => ps::p045::sol,
        46 => ps::p046::sol,
        47 => ps::p047::sol,
        48 => ps::p048::sol,
        49 => ps::p049::sol,
        50 => ps::p050::sol,
        52 => ps::p052::sol,
        53 => ps::p053::sol,
        54 => ps::p054::sol,
        55 => ps::p055::sol,
        56 => ps::p056::sol,
        57 => ps::p057::sol,
        58 => ps::p058::sol,
        59 => ps::p059::sol,
        60 => ps::p060::sol,
        61 => ps::p061::sol,
        62 => ps::p062::sol,
        63 => ps::p063::sol,
        64 => ps::p064::sol,
        65 => ps::p065::sol,
        67 => ps::p067::sol,
        68 => ps::p068::sol,
        85 => ps::p085::sol,
        97 => ps::p097::sol,
        99 => ps::p099::sol,
        100 => ps::p100::sol,
        _ => {
            eprintln!("Solution #{prob_no} not implemented yet");
            process::exit(1);
        }
    };

    time(sol);
}
