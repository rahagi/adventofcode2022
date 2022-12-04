use std::str::FromStr;

pub struct Range {
    min: u32,
    max: u32,
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');

        // lol
        let min = split.next().unwrap().parse::<u32>().unwrap();
        let max = split.next().unwrap().parse::<u32>().unwrap();

        Ok(Range::new(min, max))
    }
}

impl Range {
    pub fn new(min: u32, max: u32) -> Range {
        Range { min, max }
    }

    pub fn does_fully_overlap(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
            || other.min <= self.min && other.max >= self.max
    }

    pub fn does_overlap(&self, other: &Range) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}
