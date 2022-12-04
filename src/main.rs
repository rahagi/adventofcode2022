use aoc_2022::big_boi_runner;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day_num = &args[1];
    let part = &args[2];

    big_boi_runner(day_num, part);
}
