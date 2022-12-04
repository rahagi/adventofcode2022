/// this will be built with `--release` flag
/// and is only used for running big boi input
use aoc_2022::big_boi_runner;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day_num = &args[1];
    big_boi_runner(day_num);
}
