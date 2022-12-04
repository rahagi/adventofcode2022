pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod utils;

fn run<T>(part_one: fn(&str) -> T, part_two: fn(&str) -> T, input: &str)
where
    T: std::fmt::Display + std::fmt::Debug,
{
    for (part, func) in [part_one, part_two].iter().enumerate() {
        println!("running part {}", part + 1);

        let start = std::time::Instant::now();
        let answer = func(input);
        let duration = start.elapsed();

        println!("  answer: {:?}", answer);
        println!("  took (μs): {}", duration.as_millis());
    }
}

pub fn big_boi_runner(day_num: &str) {
    let path = format!("src/day0{}/input_big.txt", day_num);
    // check if file exists
    if std::path::Path::new(&path).exists() {
        match day_num {
            "1" => run(day01::a::a, day01::b::b, &path),
            "2" => run(day02::a::a, day02::b::b, &path),
            "3" => run(day03::a::a, day03::b::b, &path),
            "4" => run(day04::a::a, day04::b::b, &path),
            // "5" => run(day05::a::a, day05::b::b, &path),
            // "6" => run(day06::a::a, day06::b::b, &path),
            // "7" => run(day07::a::a, day07::b::b, &path),
            // would probably get filtered at this point xd
            _ => {
                eprintln!("runner: no day {day_num}");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("runner: big boi input file for day {} not found", day_num);
        std::process::exit(1);
    }
}
