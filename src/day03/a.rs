use std::collections::HashSet;

use crate::utils;

use super::helper::char2u32;

pub fn a(input_file: &str) -> u32 {
    let input = utils::file::file_to_str(input_file);
    input
        .lines()
        .map(|line| {
            let compartment_one = line[..line.len() / 2].to_string();
            let compartment_two = line[line.len() / 2..].to_string();

            let seen_c1: HashSet<char> = HashSet::from_iter(compartment_one.chars());
            let seen_c2: HashSet<char> = HashSet::from_iter(compartment_two.chars());

            let intersect = seen_c1.intersection(&seen_c2).next().unwrap();
            char2u32(*intersect)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_03_a() {
        assert_eq!(a("src/day03/input_example.txt"), 157);
        println!("Answer: {}", a("src/day03/input.txt"));
    }
}
