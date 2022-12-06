use crate::utils;
use std::collections::HashSet;

const WINDOW_LEN: usize = 4;

pub fn a(input_file: &str) -> usize {
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
    use super::a;
    #[test]
    fn test_06_a() {
        assert_eq!(a("src/day06/input_example.txt"), 7);
        println!("Answer: {}", a("src/day06/input.txt"));
    }
}
