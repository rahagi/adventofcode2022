use super::coord::Coord;
use core::str::FromStr;

pub struct Move {
    pub dir: Coord,
    pub n: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = s.split_whitespace();
        let dir = part.next().unwrap();
        let num = part.next().unwrap().parse::<usize>().unwrap();
        match dir {
            "U" => Ok(Move {
                dir: Coord::new(0, 1),
                n: num,
            }),
            "D" => Ok(Move {
                dir: Coord::new(0, -1),
                n: num,
            }),
            "L" => Ok(Move {
                dir: Coord::new(-1, 0),
                n: num,
            }),
            "R" => Ok(Move {
                dir: Coord::new(1, 0),
                n: num,
            }),
            _ => Err(()),
        }
    }
}
