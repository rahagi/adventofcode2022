use crate::{day01, day02, day03, day04, day05};

type Runner<T> = fn(&str) -> T;

fn run_solver<T>(f: Runner<T>, input_file: &str)
where
    T: std::fmt::Display + std::fmt::Debug,
{
    println!("answer: {}", f(input_file));
}

pub fn run(day_num: &str, part: &str) {
    fn err(msg: &str) {
        eprintln!("runner: {msg}");
        std::process::exit(1);
    }

    let err_part = || err("there is only 2 parts...");
    let err_day = || err(&format!("big boi input for day {day_num} not found"));

    let input_file = format!("src/day0{}/input_big.txt", day_num);
    if std::path::Path::new(&input_file).exists() {
        match day_num {
            "1" => match part {
                "1" => run_solver(day01::a::a, &input_file),
                "2" => run_solver(day01::b::b, &input_file),
                _ => err_part(),
            },
            "2" => match part {
                "1" => run_solver(day02::a::a, &input_file),
                "2" => run_solver(day02::b::b, &input_file),
                _ => err_part(),
            },
            "3" => match part {
                "1" => run_solver(day03::a::a, &input_file),
                "2" => run_solver(day03::b::b, &input_file),
                _ => err_part(),
            },
            "4" => match part {
                "1" => run_solver(day04::a::a, &input_file),
                "2" => run_solver(day04::b::b, &input_file),
                _ => err_part(),
            },
            "5" => match part {
                "1" => run_solver(day05::a::a, &input_file),
                "2" => run_solver(day05::b::b, &input_file),
                _ => err_part(),
            },
            _ => unreachable!(),
        }
    } else {
        err_day();
    }
}
