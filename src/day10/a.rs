use super::cpu::Cpu;
use crate::utils;

pub fn a(input_file: &str) -> isize {
    let input = utils::file::file_to_str(input_file);
    let mut cpu = Cpu::new();
    cpu.load(&input);

    let mut signal_sum: isize = 0;
    loop {
        cpu.next_cycle();
        match cpu.cycle {
            220 => {
                signal_sum += cpu.x_register * 220;
                break;
            }
            180 => {
                signal_sum += cpu.x_register * 180;
            }
            140 => {
                signal_sum += cpu.x_register * 140;
            }
            100 => {
                signal_sum += cpu.x_register * 100;
            }
            60 => {
                signal_sum += cpu.x_register * 60;
            }
            20 => {
                signal_sum += cpu.x_register * 20;
            }
            _ => {}
        }
    }

    signal_sum
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_10_a() {
        assert_eq!(a("src/day10/input_example.txt"), 13140);
        println!("Answer: {}", a("src/day10/input.txt"));
    }
}
