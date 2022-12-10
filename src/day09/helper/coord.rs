use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    pub fn distance_between(&self, other: Coord) -> f32 {
        let abs_distance = (self.x - other.x).abs() + (self.y - other.y).abs();
        if self.is_diagonal(other) {
            abs_distance as f32 - 0.5f32
        } else {
            abs_distance as f32
        }
    }

    pub fn is_diagonal(&self, other: Coord) -> bool {
        self.x != other.x && self.y != other.y
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
