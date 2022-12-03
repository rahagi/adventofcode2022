use crate::utils;
use std::str::FromStr;

use super::game::{rps::Rps, shape::Shape};

pub fn a(input_file: &str) -> i32 {
    let input = utils::file::file_to_str(input_file);
    input
        .lines()
        .map(|round| {
            let mut rps = round.split_whitespace();
            let opponent = Shape::from_str(rps.next().unwrap()).unwrap();
            let player = Shape::from_str(rps.next().unwrap()).unwrap();

            let mut game = Rps::new(player, opponent);
            game.start();
            game.player_points_gained
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_02_a() {
        assert_eq!(a("src/day02/input_example.txt"), 15);
        println!("Answer: {}", a("src/day02/input.txt"));
    }
}
