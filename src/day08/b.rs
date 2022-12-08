use super::helper::parse_forest;
use crate::utils;

/// forsenPossessed
fn scenic_score(current_idx: (usize, usize), forest: &[Vec<usize>]) -> usize {
    let (x, y) = current_idx;

    let current = forest[y][x];

    let mut rc = 0;
    let right = forest[y].iter().skip(x + 1);
    for r in right {
        rc += 1;
        if *r >= current {
            break;
        }
    }

    let mut dc = 0;
    let down = forest.iter().skip(y + 1).map(|col| &col[x]);
    for d in down {
        dc += 1;
        if *d >= current {
            break;
        }
    }

    let mut lc = 0;
    let left = forest[y].iter().take(x).rev();
    for l in left {
        lc += 1;
        if *l >= current {
            break;
        }
    }

    let mut uc = 0;
    let up = forest.iter().take(y).rev().map(|col| &col[x]);
    for u in up {
        uc += 1;
        if *u >= current {
            break;
        }
    }

    rc * dc * lc * uc
}

pub fn b(input_file: &str) -> usize {
    let input = utils::file::file_to_str(input_file);
    let forest = parse_forest(&input);

    forest
        .iter()
        .skip(1)
        .take(forest.len() - 2)
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .skip(1)
                .take(row.len() - 2)
                .enumerate()
                .map(|(x, _)| scenic_score((x + 1, y + 1), &forest))
                .collect::<Vec<usize>>()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::b;
    #[test]
    fn test_08_b() {
        assert_eq!(b("src/day08/input_example.txt"), 8);
        println!("Answer: {}", b("src/day08/input.txt"));
    }
}
