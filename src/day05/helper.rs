use std::{cell::RefCell, collections::BTreeMap};

pub struct Move {
    pub src: usize,
    pub dst: usize,
    pub len: usize,
}

pub fn parse_stacks(input: &str) -> BTreeMap<usize, RefCell<Vec<char>>> {
    let mut stacks: BTreeMap<usize, RefCell<Vec<char>>> = BTreeMap::new();
    let stack_str = input.split("\n\n").next().unwrap();
    stack_str
        .lines()
        .take_while(|line| !line.contains(char::is_numeric))
        .for_each(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .for_each(|(i, r_crate)| {
                    if let Some(c) = r_crate.get(1).copied() {
                        if c != ' ' {
                            stacks.entry(i).or_default().borrow_mut().insert(0, c);
                        }
                    }
                })
        });
    stacks
}

pub fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let move_str = input.split("\n\n").nth(1).unwrap();
    move_str.lines().for_each(|line| {
        let ins = line.split_whitespace().collect::<Vec<&str>>();

        // yolo
        let src: usize = ins[3].parse().unwrap();
        let dst: usize = ins[5].parse().unwrap();
        let len = ins[1].parse().unwrap();

        moves.push(Move {
            src: src - 1,
            dst: dst - 1,
            len,
        });
    });
    moves
}

#[cfg(test)]
mod tests {
    use super::{parse_moves, parse_stacks};
    use crate::utils;
    #[test]
    fn test_parse_stack() {
        let input = utils::file::file_to_str("src/day05/input_example.txt");
        let stacks = parse_stacks(&input);
        assert_eq!(stacks.len(), 3);
        println!("{:?}", stacks);
    }

    #[test]
    fn test_parse_moves() {
        let input = utils::file::file_to_str("src/day05/input_example.txt");
        assert_eq!(parse_moves(&input).len(), 4);
    }
}
