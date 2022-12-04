use std::str::FromStr;

use crate::utils;

use super::range::Range;

pub fn b(input_file: &str) -> u32 {
    let input = utils::file::file_to_str(input_file);
    input
        .lines()
        .filter(|pair| {
            let mut assignment = pair.split(',');

            let elve_one = assignment.next().unwrap();
            let elve_two = assignment.next().unwrap();

            let elve_one_range = Range::from_str(elve_one).unwrap();
            let elve_two_range = Range::from_str(elve_two).unwrap();

            elve_one_range.does_overlap(&elve_two_range)
        })
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_04_b() {
        assert_eq!(b("src/day04/input_example.txt"), 4);
        println!("Answer: {}", b("src/day04/input.txt"));
    }
}
