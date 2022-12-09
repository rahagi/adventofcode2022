use super::coord::Coord;
use std::collections::HashSet;

type Id = usize;

pub struct Rope {
    head: Id,
    tail: Id,
    head_last_trace: Id,
    coords: Vec<Coord>,
    pub visited_by_tail: HashSet<Coord>,
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            head: 0,
            tail: 0,
            head_last_trace: 0,
            coords: vec![Coord::new(0, 0)],
            visited_by_tail: HashSet::new(),
        }
    }

    pub fn make_move(&mut self, coord: Coord) {
        let new_coord = self.coords[self.head] + coord;
        self.coords.push(new_coord);
        let new_id = self.coords.len() - 1;

        self.head = new_id;
        if self.distance_between() >= 2.0 {
            self.tail = self.head_last_trace;
        }

        self.visited_by_tail.insert(self.coords[self.tail]);
        self.head_last_trace = self.head;
    }

    fn distance_between(&self) -> f32 {
        let head = self.coords[self.head];
        let tail = self.coords[self.tail];
        let abs_distance = (head.x - tail.x).abs() + (head.y - tail.y).abs();
        if self.is_diagonal() {
            abs_distance as f32 - 0.5f32
        } else {
            abs_distance as f32
        }
    }

    fn is_diagonal(&self) -> bool {
        self.coords[self.head].x != self.coords[self.tail].x
            && self.coords[self.head].y != self.coords[self.tail].y
    }
}
