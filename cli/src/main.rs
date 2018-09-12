extern crate clap;
extern crate andaluz_core;

use clap::{Arg, App};
use andaluz_core::board::{Board, BoardOptions};
use andaluz_core::heuristic::bruteforce::BruteForce;
use andaluz_core::heuristic::attacksum::AttackSum;
use andaluz_core::heuristic::horse::Horse;

fn main() {
    let matches = App::new("Andaluz CLI")
        .version("1.0.0")
        .author("Victor L. <vhrlopes@gmail.com>")
        .arg(Arg::with_name("cols")
             .short("c")
             .long("Columns of the board")
             .takes_value(true))
        .arg(Arg::with_name("max-jumps")
             .short("m")
             .long("Maximum number of jumps before fail")
             .takes_value(true))
        .arg(Arg::with_name("bruteforce")
             .short("b")
             .long("Weight of bruteforce heuristic")
             .takes_value(true))
        .arg(Arg::with_name("attacksum")
             .short("a")
             .long("Weight of attacksum heuristic")
             .takes_value(true))
        .arg(Arg::with_name("horse")
             .short("h")
             .long("Weight of horse heuristic")
             .takes_value(true))
        .get_matches();

    let cols: usize = matches
        .value_of("cols")
        .unwrap_or("8")
        .parse()
        .unwrap_or(8);

    let max_jumps: u64 = matches
        .value_of("max-jumps")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    let bruteforce: f64 = matches
        .value_of("bruteforce")
        .unwrap_or("0.0")
        .parse()
        .unwrap_or(0.0);

    let attacksum: f64 = matches
        .value_of("attacksum")
        .unwrap_or("0.0")
        .parse()
        .unwrap_or(0.0);

    let horse: f64 = matches
        .value_of("horse")
        .unwrap_or("0.0")
        .parse()
        .unwrap_or(0.0);

    println!("Starting processing with cols = {}, maxjumps = {} and weights:", cols, max_jumps);
    println!("BruteForce: {}", bruteforce);
    println!("AttackSum: {}", attacksum);
    println!("Horse: {}", horse);

    let options = BoardOptions {
        cols,
        max_jumps,
    };

    let mut board = Board::from_options(options);
    let max_weigth = bruteforce + attacksum + horse;
    
    if bruteforce > 0.0 {
        board.inject_heuristic(BruteForce {},
                               bruteforce / max_weigth);
    }
    if attacksum > 0.0 {
        board.inject_heuristic(AttackSum {},
                               attacksum / max_weigth);
    }
    if horse > 0.0 {
        board.inject_heuristic(Horse {},
                               horse / max_weigth);
    }

    let solved = board.solve();
    let jumps = board.jumps;

    if solved {
        println!("Solved with {} jumps!", jumps);
    } else {
        println!("Solution failed after {} jumps!", jumps);
    }
}
