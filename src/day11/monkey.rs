use std::str::FromStr;

pub type Id = usize;

pub enum Operation {
    Multiply(Option<usize>),
    Add(Option<usize>),
}

pub struct Monkey {
    pub operation: Operation,
    pub stack: Vec<usize>,
    pub test: usize,
    pub next_if_true: Id,
    pub next_if_false: Id,
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            operation: Operation::Multiply(None),
            stack: Vec::new(),
            test: 0,
            next_if_true: 0,
            next_if_false: 0,
        }
    }

    fn load_stack(&mut self, stack: &str) {
        let parts = stack.split(": ").nth(1).unwrap();
        self.stack = parts
            .rsplit(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
    }

    fn load_operation(&mut self, operation: &str) {
        let parts = operation.split(": ").nth(1).unwrap();
        let arg = parts.split(|c| c == '*' || c == '+').nth(1).unwrap().trim();

        if let Ok(arg) = arg.parse::<usize>() {
            if operation.contains('*') {
                self.operation = Operation::Multiply(Some(arg));
            } else {
                self.operation = Operation::Add(Some(arg));
            }
        } else if operation.contains('*') {
            self.operation = Operation::Multiply(None);
        } else {
            self.operation = Operation::Add(None);
        }
    }

    fn load_test(&mut self, test: &str) {
        let parts = test.split(' ').last().unwrap();
        self.test = parts.parse().unwrap();
    }

    fn load_next_if_true(&mut self, next_if_true: &str) {
        let parts = next_if_true.split(' ').last().unwrap();
        self.next_if_true = parts.parse().unwrap();
    }

    fn load_next_if_false(&mut self, next_if_false: &str) {
        let parts = next_if_false.split(' ').last().unwrap();
        self.next_if_false = parts.parse().unwrap();
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkey = Monkey::new();
        let mut lines = s.lines().skip(1);

        let stack = lines.next().unwrap();
        let operation = lines.next().unwrap();
        let test = lines.next().unwrap();
        let next_if_true = lines.next().unwrap();
        let next_if_false = lines.next().unwrap();

        monkey.load_stack(stack);
        monkey.load_operation(operation);
        monkey.load_test(test);
        monkey.load_next_if_true(next_if_true);
        monkey.load_next_if_false(next_if_false);

        Ok(monkey)
    }
}
