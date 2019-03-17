use andaluz_core::board::Board;
use andaluz_core::solver::Solver;
use andaluz_core::heuristic_implementation::HeuristicImplementation;
use andaluz_core::heuristic_implementation::prioritizecenter::PrioritizeCenter;
use clap::{App, Arg};

const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let matches = App::new(NAME.unwrap())
        .version(VERSION.unwrap())
        .author(AUTHORS.unwrap())
        .about(DESCRIPTION.unwrap())
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Set the i/o format")
                .takes_value(true)
                .possible_values(&["bits", "decimal", "hex", "pretty"])
                .default_value("pretty"),
        )
        .arg(
            Arg::with_name("cols")
                .short("c")
                .long("columns")
                .value_name("COLS")
                .help("Set the number of columns")
                .takes_value(true)
                .default_value("8"),
        )
        .arg(
            Arg::with_name("max_jumps")
                .short("m")
                .long("max_jumps")
                .value_name("MAXJUMPS")
                .help("Set the maximum number of jumps to be performed")
                .takes_value(true)
                .default_value("100000"),
        )
        .get_matches();

    let cols: usize = matches.value_of("cols").unwrap().parse().unwrap();
    let mut board = Board::new(cols);
    let original = board.clone();

    let max_jumps: u32 = matches.value_of("max_jumps").unwrap().parse().unwrap();
    let mut solver = Solver::new();
    solver.set_max_jumps(max_jumps);

    // TODO - Define weigths via cli interface
    solver.push_heuristic(PrioritizeCenter::new(1.0));

    let result = solver.solve(&mut board).unwrap();

    if result.is_solved() {
        match matches.value_of("format").unwrap() {
            "bits" => {
                println!("{},{},{}", original.to_string(), result.get_jumps(), board.to_string());
            },
            "decimal" => {
                println!("{:?},{},{:?}", original.get_signature(), result.get_jumps(), board.get_signature());
            },
            "hex" => {
                println!("{:x?},{},{:x?}", original.get_signature(), result.get_jumps(), board.get_signature());
            },
            "pretty" => {
                println!("Heuristics: {}", result.get_heuristics_description());
                println!("Jumps: {}", result.get_jumps());
                println!("Signature: {:?}", board.get_signature());
                println!("Bits: {}", board.to_string());
                println!("");
                println!("From:");
                println!("{}", original.to_multiline_string());
                println!("");
                println!("To:");
                println!("{}", board.to_multiline_string());
            },
            _ => {},
        };
    }
}
