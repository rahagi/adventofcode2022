use crate::utils;

pub fn b(input_file: &str) -> i32 {
    let input = utils::file::file_to_str(input_file);
    let mut all_sum = input
        .split("\n\n")
        .into_iter()
        .map(|foods| {
            foods
                .lines()
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    all_sum.sort();
    all_sum[all_sum.len() - 3..].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_01_b() {
        assert_eq!(b("src/day01/input_example.txt"), 45000);
        println!("Answer: {}", b("src/day01/input.txt"));
    }
}
