use super::monkey::{Id, Monkey, Operation};
use crate::utils;
use std::{cell::RefCell, collections::BTreeMap, str::FromStr};

const ROUND_REQUIRED: usize = 20;

pub fn a(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);

    let mut monkeys: BTreeMap<Id, RefCell<Monkey>> = BTreeMap::new();
    input.split("\n\n").enumerate().for_each(|(id, monkey)| {
        let monkey = Monkey::from_str(monkey).unwrap();
        monkeys.insert(id, RefCell::new(monkey));
    });
    let mut inspect_count = vec![0; monkeys.len()];

    let mut round = 1;
    while round <= ROUND_REQUIRED {
        monkeys.iter().for_each(|(id, monkey)| {
            let mut monkey = monkey.borrow_mut();

            while let Some(worry_level) = monkey.stack.pop() {
                let mut item_value = match monkey.operation {
                    Operation::Multiply(Some(arg)) => worry_level * arg,
                    Operation::Multiply(None) => worry_level * worry_level,
                    Operation::Add(Some(arg)) => worry_level + arg,
                    Operation::Add(None) => worry_level + worry_level,
                };
                item_value /= 3;

                let next_id = if item_value % monkey.test == 0 {
                    monkey.next_if_true
                } else {
                    monkey.next_if_false
                };

                monkeys
                    .get(&next_id)
                    .unwrap()
                    .borrow_mut()
                    .stack
                    .push(item_value);
                inspect_count[*id] += 1;
            }
        });
        round += 1;
    }

    inspect_count.sort();
    inspect_count[inspect_count.len() - 2..].iter().product()
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_11_a() {
        assert_eq!(a("src/day11/input_example.txt"), 10605);
        println!("Answer: {}", a("src/day11/input.txt"));
    }
}
