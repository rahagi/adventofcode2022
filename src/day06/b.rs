use crate::utils;
use std::collections::HashSet;

const WINDOW_LEN: usize = 14;

pub fn b(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(WINDOW_LEN)
        .map(HashSet::from_iter)
        .take_while(|set: &HashSet<&char>| set.len() != WINDOW_LEN)
        .count()
        + WINDOW_LEN
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_06_b() {
        assert_eq!(b("src/day06/input_example.txt"), 19);
        println!("Answer: {}", b("src/day06/input.txt"));
    }
}
