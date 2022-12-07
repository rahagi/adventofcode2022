use std::collections::HashMap;

pub fn parse_commands(input: &str) -> HashMap<String, usize> {
    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut current_dir = vec![];

    input.lines().for_each(|term_output| {
        if term_output.starts_with('$') {
            let mut parts = term_output.split_whitespace().skip(1);
            let cmd = parts.next().unwrap();
            let args = parts.next().unwrap_or("");
            if cmd == "cd" {
                if args == ".." {
                    current_dir.pop();
                } else {
                    current_dir.push(args);
                }
            }
        } else if term_output.starts_with(char::is_numeric) {
            let mut parts = term_output.split_whitespace();
            let size = parts.next().unwrap().parse::<usize>().unwrap();
            current_dir.iter().enumerate().for_each(|(i, _)| {
                let path = current_dir[..i + 1].join("");
                sizes.entry(path).and_modify(|s| *s += size).or_insert(size);
            });
        }
    });

    sizes
}

#[cfg(test)]
mod tests {
    use super::parse_commands;
    use crate::utils;
    #[test]
    fn test_parse_nodes() {
        let input = utils::file::file_to_str("src/day07/input_example.txt");
        let sizes = parse_commands(&input);
        assert_eq!(sizes.get("/").unwrap(), &48381165);
        assert_eq!(sizes.get("/a").unwrap(), &94853);
        assert_eq!(sizes.get("/ae").unwrap(), &584);
        assert_eq!(sizes.get("/d").unwrap(), &24933642);
    }
}
