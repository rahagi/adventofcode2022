use crate::utils;

pub fn a(input_file: &str) -> i32 {
    let input = utils::file::file_to_str(input_file);
    let mut max = 0;
    for foods in input.split("\n\n").into_iter() {
        let sum = foods.lines().map(|x| x.parse::<i32>().unwrap()).sum();
        if sum > max {
            max = sum;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_01_a() {
        assert_eq!(a("src/day01/input_example.txt"), 24000);
        println!("Answer: {}", a("src/day01/input.txt"));
    }
}
