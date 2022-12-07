use super::helper::parse_commands;
use crate::utils;

pub fn a(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let sizes = parse_commands(&input);

    sizes.values().into_iter().filter(|s| **s <= 100000).sum()
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_07_a() {
        assert_eq!(a("src/day07/input_example.txt"), 95437);
        println!("Answer: {}", a("src/day07/input.txt"));
    }
}
