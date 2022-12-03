use std::collections::HashSet;

use crate::utils;

use super::helper::char2u32;

pub fn b(input_file: &str) -> u32 {
    let input = utils::file::file_to_str(input_file);
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|sack_group| {
            let first_set: HashSet<char> = HashSet::from_iter(sack_group[0].chars());
            sack_group[1..]
                .iter()
                .fold(first_set, |acc, sack| {
                    let sack_set: HashSet<char> = HashSet::from_iter(sack.chars());
                    acc.intersection(&sack_set).cloned().collect()
                })
                .iter()
                .map(|c| char2u32(*c))
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_03_b() {
        assert_eq!(b("src/day03/input_example.txt"), 70);
        println!("Answer: {}", b("src/day03/input.txt"));
    }
}
