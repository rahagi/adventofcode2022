pub fn parse_forest(input: &str) -> Vec<Vec<usize>> {
    let mut forest: Vec<Vec<usize>> = vec![];
    for line in input.lines() {
        forest.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }
    forest
}

#[cfg(test)]
mod tests {
    use super::parse_forest;
    use crate::utils;

    #[test]
    fn test_parse_forest() {
        let input = utils::file::file_to_str("src/day08/input_example.txt");
        let forest = parse_forest(&input);
        assert_eq!(forest.len(), 5);
        assert_eq!(forest[0].len(), 5);
        println!("{:?}", forest);
    }
}
