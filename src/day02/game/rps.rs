use super::shape::Shape;

const WIN: i32 = 6;
const LOSE: i32 = 0;
const DRAW: i32 = 3;

pub struct Rps {
    player: Shape,
    opponent: Shape,
    pub player_points_gained: i32,
}

impl Rps {
    pub fn new(player: Shape, opponent: Shape) -> Self {
        Self {
            player,
            opponent,
            player_points_gained: 0,
        }
    }

    pub fn start(&mut self) {
        let mut points = 0;
        match (&self.player, &self.opponent) {
            (Shape::Rock, Shape::Scissors) => points += WIN,
            (Shape::Paper, Shape::Rock) => points += WIN,
            (Shape::Scissors, Shape::Paper) => points += WIN,
            (Shape::Rock, Shape::Paper) => points += LOSE,
            (Shape::Paper, Shape::Scissors) => points += LOSE,
            (Shape::Scissors, Shape::Rock) => points += LOSE,
            _ => points += DRAW,
        }
        self.player_points_gained += points + self.player.score()
    }
}
