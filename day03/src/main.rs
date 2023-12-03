#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./day03/input.txt").unwrap();
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
        assert_eq!(
            solve_part_1(
                "467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598.."
            ),
            4361
        );
    }

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(solve_part_2(""), 0);
    }
}
