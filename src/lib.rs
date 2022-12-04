pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod utils;

pub fn big_boi_runner(day_num: &str, part: &str) {
    let path = format!("src/day0{}/input_big.txt", day_num);
    let result = match part {
        "a" => match day_num {
            "3" => day03::a::a(&path),
            _ => panic!("day {day_num} does not have big boi input"),
        },
        "b" => match day_num {
            "3" => day03::b::b(&path),
            _ => panic!("day {day_num} does not have big boi input"),
        },
        _ => unreachable!(),
    };
    println!("day {day_num} {part} big: {result}");
}
