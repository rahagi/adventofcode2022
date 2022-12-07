use super::helper::parse_commands;
use crate::utils;

const DISK_CAP: usize = 70000000;
const MIN_UNUSED_REQUIRED: usize = 30000000;

pub fn b(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let sizes = parse_commands(&input);

    let current_used = sizes.get("/").unwrap();
    let current_free = DISK_CAP - current_used;
    let desired_free = MIN_UNUSED_REQUIRED - current_free;

    *sizes
        .values()
        .into_iter()
        .filter(|s| **s > desired_free)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_07_b() {
        assert_eq!(b("src/day07/input_example.txt"), 24933642);
        println!("Answer: {}", b("src/day07/input.txt"));
    }
}
