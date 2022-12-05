use super::helper;
use crate::utils;

pub fn a(input_file: &str) -> String {
    let input = utils::file::file_to_str(input_file);
    let stacks = helper::parse_stacks(&input);
    let instructions = helper::parse_moves(&input);

    instructions.iter().for_each(|ins| {
        let mut src = stacks.get(&ins.src).unwrap().borrow_mut();
        src.drain(..ins.len).for_each(|c| {
            stacks.get(&ins.dst).unwrap().borrow_mut().insert(0, c);
        });
    });

    stacks
        .iter()
        .map(|(_, v)| *v.borrow().first().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_05_a() {
        assert_eq!(a("src/day05/input_example.txt"), "CMZ");
        println!("Answer: {}", a("src/day05/input.txt"));
    }
}
