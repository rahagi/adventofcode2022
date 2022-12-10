use super::helper::{rmove::Move, rope::Rope};
use crate::utils;

pub fn b(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let mut rope = Rope::new();

    input.lines().for_each(|ins| {
        let rmove = ins.parse::<Move>().unwrap();
        (0..rmove.n).for_each(|_| {
            rope.make_move(rmove.dir);
        });
    });

    rope.visited_by_tail.len()
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_09_b() {
        assert_eq!(b("src/day09/input_example.txt"), 1);
        assert_eq!(b("src/day09/input_example_big.txt"), 36);
        println!("Answer: {}", b("src/day09/input.txt"));
    }
}
