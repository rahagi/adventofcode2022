use crate::utils;

pub fn a(input_file: &str) {}

#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_{{DAYNUM}}_a() {
        assert_eq!(a("src/day{{DAYNUM}}/input_example.txt"), 0);
        println!("Answer: {}", a("src/day{{DAYNUM}}/input.txt"));
    }
}
