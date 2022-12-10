use super::coord::Coord;
use std::collections::HashSet;

const KNOT_LENGTH: usize = 9;

type Id = usize;

pub struct Rope {
    head: Id,
    tails: [Id; KNOT_LENGTH],
    head_last_trace: Id,
    coords: Vec<Coord>,
    pub visited_by_tail: HashSet<Coord>,
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            head: 0,
            tails: [0; KNOT_LENGTH],
            head_last_trace: 0,
            coords: vec![Coord::new(0, 0)],
            visited_by_tail: HashSet::new(),
        }
    }

    pub fn make_move(&mut self, dir: Coord) {
        let new_coord = self.coords[self.head] + dir;
        self.coords.push(new_coord);
        let new_id = self.coords.len() - 1;

        self.head = new_id;
        if self.coords[self.head].distance_between(self.coords[self.tails[0]]) >= 2.0 {
            self.tails[0] = self.head_last_trace;
        }

        (1..KNOT_LENGTH).for_each(|i| {
            if self.coords[self.tails[i - 1]].distance_between(self.coords[self.tails[i]]) >= 2.0 {
                let tail_predecessor = self.tails[i - 1];
                let tail = self.tails[i];

                let diff_x = self.coords[tail_predecessor].x - self.coords[tail].x;
                let diff_y = self.coords[tail_predecessor].y - self.coords[tail].y;

                let new_tail_coord = self.coords[tail] + Coord::new(diff_x.signum(), diff_y.signum());
                self.coords.push(new_tail_coord);
                self.tails[i] = self.coords.len() - 1;
            }
        });

        self.visited_by_tail
            .insert(self.coords[self.tails[KNOT_LENGTH - 1]]);
        self.head_last_trace = self.head;
    }
}
