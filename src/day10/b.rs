use crate::{day10::cpu::Cpu, utils};

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

pub fn b(input_file: &str) {
    let input = utils::file::file_to_str(input_file);
    let mut cpu = Cpu::new();
    cpu.load(&input);

    let mut row: usize = 0;
    loop {
        let x_reg = cpu.x_register;
        let sprite = x_reg - 1..=x_reg + 1;

        if sprite.contains(&((row % WIDTH) as isize)) {
            print!("#");
        } else {
            print!(".");
        }

        row += 1;
        if row % WIDTH == 0 {
            println!();
        }

        if !cpu.next_cycle() || cpu.cycle >= WIDTH * HEIGHT {
            println!();
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_10_b() {
        // ##..##..##..##..##..##..##..##..##..##..
        // ###...###...###...###...###...###...###.
        // ####....####....####....####....####....
        // #####.....#####.....#####.....#####.....
        // ######......######......######......####
        // #######.......#######.......#######.....
        b("src/day10/input_example.txt");

        println!();

        // ####.####..##..####.####.#....#..#.####.
        // #....#....#..#....#.#....#....#..#.#....
        // ###..###..#......#..###..#....####.###..
        // #....#....#.....#...#....#....#..#.#....
        // #....#....#..#.#....#....#....#..#.#....
        // #....####..##..####.####.####.#..#.####
        b("src/day10/input.txt");
    }
}
