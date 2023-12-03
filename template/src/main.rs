#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./{{project-name}}/input.txt").unwrap();
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

mod part_1 {
    pub(crate) fn solve(input: &str) -> u32 {
        0
    }
}

mod part_2 {
    pub(crate) fn solve(input: &str) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part_1_test() {
        assert_eq!(part_1::solve(""), 0);
    }

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(part_2::solve(""), 0);
    }
}
