use super::helper::parse_forest;
use crate::utils;

fn is_visible(current_idx: (usize, usize), forest: &[Vec<usize>]) -> bool {
    let (x, y) = current_idx;

    let current = forest[y][x];
    let right = forest[y].iter().skip(x + 1);
    let down = forest.iter().skip(y + 1).map(|col| &col[x]);
    let left = forest[y].iter().take(x);
    let up = forest.iter().take(y).map(|col| &col[x]);

    *right.max().unwrap() < current
        || *down.max().unwrap() < current
        || *left.max().unwrap() < current
        || *up.max().unwrap() < current
}

pub fn a(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let forest = parse_forest(&input);

    forest
        .iter()
        .skip(1)
        .take(forest.len() - 2)
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .skip(1)
                .take(row.len() - 2)
                .enumerate()
                .filter(|(x, _)| is_visible((*x + 1, y + 1), &forest))
                .count()
        })
        .sum::<usize>()
        + forest.len() * 2
        + forest[0].len() * 2
        - 4
}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_08_a() {
        assert_eq!(a("src/day08/input_example.txt"), 21);
        println!("Answer: {}", a("src/day08/input.txt"));
    }
}
