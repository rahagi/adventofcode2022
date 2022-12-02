use std::str::FromStr;

pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(&self) -> i32 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "A" => Ok(Shape::Rock),
            "Y" | "B" => Ok(Shape::Paper),
            "Z" | "C" => Ok(Shape::Scissors),
            _ => Err("invalid input"),
        }
    }
}
