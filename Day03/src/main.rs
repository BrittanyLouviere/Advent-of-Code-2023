#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(_input: &str) -> u32 {
    0
}

fn solve_part_2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part_1_test() {
        assert_eq!(solve_part_1(""), 0);
    }

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(solve_part_2(""), 0);
    }
}
