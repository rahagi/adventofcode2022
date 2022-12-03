use std::str::FromStr;

use crate::utils;

use super::game::{rps::Rps, shape::Shape};

pub fn b(input_file: &str) -> i32 {
    let input = utils::file::file_to_str(input_file);
    input
        .lines()
        .map(|round| {
            let mut rps = round.split_whitespace();
            let opponent = Shape::from_str(rps.next().unwrap()).unwrap();
            let player = {
                let code = rps.next().unwrap();
                match code {
                    "X" => opponent.wins_against(),
                    "Y" => opponent,
                    "Z" => opponent.loses_to(),
                    _ => unreachable!(),
                }
            };

            let mut game = Rps::new(player, opponent);
            game.start();
            game.player_points_gained
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_02_b() {
        assert_eq!(b("src/day02/input_example.txt"), 12);
        println!("Answer: {}", b("src/day02/input.txt"));
    }
}
