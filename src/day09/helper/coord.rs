use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
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
