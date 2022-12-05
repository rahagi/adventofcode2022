/// this will be built with `--release` flag
/// and is only used for running big boi input
use aoc_2022::utils::big_boi;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("usage: {} <day> <part>", args[0]);
        std::process::exit(1);
    }

    let day_num = &args[1];
    let part = &args[2];

    big_boi::run(day_num, part);
}
