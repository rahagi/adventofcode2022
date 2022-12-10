use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Add(isize),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let instruction = parts.next().unwrap();
        let argument = parts.next().unwrap_or("0").parse::<isize>().unwrap();
        match instruction {
            "addx" => Ok(Instruction::Add(argument)),
            "noop" => Ok(Instruction::Noop),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    instructions: Vec<Instruction>,
    buffer: Vec<isize>,
    cycle_since_last_add: usize,
    pub x_register: isize,
    pub cycle: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            instructions: Vec::new(),
            buffer: Vec::new(),
            cycle_since_last_add: 0,
            x_register: 1,
            cycle: 1,
        }
    }

    pub fn load(&mut self, instructions: &str) {
        self.instructions = instructions
            .lines()
            .map(|ins| ins.parse::<Instruction>().unwrap())
            .rev()
            .collect();
    }

    pub fn next_cycle(&mut self) {
        if self.instructions.is_empty() {
            return;
        }

        self.cycle += 1;

        let instruction = &self.instructions[self.instructions.len() - 1];
        match instruction {
            Instruction::Add(x) => {
                self.cycle_since_last_add += 1;
                if self.buffer.is_empty() {
                    self.buffer.push(*x);
                } else if self.cycle_since_last_add == 2 {
                    self.x_register += self.buffer.pop().unwrap();
                    self.consume_instructions();
                }
            }
            Instruction::Noop => {
                self.consume_instructions();
            }
        }
    }

    fn consume_instructions(&mut self) {
        self.instructions.pop();
        self.cycle_since_last_add = 0;
    }
}
