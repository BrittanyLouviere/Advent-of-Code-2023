#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./day03/input.txt").unwrap();
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

mod part_1 {
    pub(crate) fn solve(input: &str) -> u32 {
        // sum all numbers adjacent to a symbol
        let mut sum = 0;

        let lines: Vec<&str> = input.lines().collect();
        let padding_line = &".".repeat(lines[0].len());
        for i in 0..lines.len() {
            let prev = format!(".{}.", if i == 0 { padding_line } else { lines[i - 1] });
            let curr = format!(".{}.", lines[i]);
            let next = format!(
                ".{}.",
                if i == lines.len() - 1 {
                    padding_line
                } else {
                    lines[i + 1]
                }
            );
            sum += check_line(&prev, &curr, &next);
        }
        sum
    }

    fn check_line(prev: &str, curr: &str, next: &str) -> u32 {
        let numbers: Vec<&str> = curr
            .split(|ch: char| !ch.is_numeric())
            .filter(|x| x != &"")
            .collect();
        let mut sum = 0;
        let mut start_index = 0;

        for number in numbers {
            let index = curr[start_index..].find(number).unwrap() + start_index;

            let mut test_set = String::new();
            let start = index - 1;
            let end = index + number.len() + 1;
            for line in [prev, curr, next] {
                test_set.push_str(&line[start..end]);
            }

            if test_set.chars().any(|x| !x.is_numeric() && x != '.') {
                sum += number.parse::<u32>().unwrap();
            }
            start_index = index + number.len();
        }
        sum
    }
}

mod part_2 {
    pub(crate) fn solve(input: &str) -> u32 {
        // find `*` that is adjacent to exactly two numbers
        // multiply numbers together
        // sum all these together
        let mut sum = 0;

        let lines: Vec<&str> = input.lines().collect();
        let padding_line = &".".repeat(lines[0].len());
        for i in 0..lines.len() {
            let prev = format!(".{}.", if i == 0 { padding_line } else { lines[i - 1] });
            let curr = format!(".{}.", lines[i]);
            let next = format!(
                ".{}.",
                if i == lines.len() - 1 {
                    padding_line
                } else {
                    lines[i + 1]
                }
            );
            sum += check_line(&prev, &curr, &next);
        }
        sum
    }

    fn check_line(prev: &str, curr: &str, next: &str) -> u32 {
        let star_indeces: Vec<(usize, &str)> = curr.match_indices('*').collect();
        let mut sum = 0;

        let mut numbers = vec![];
        for line in [prev, curr, next] {
            numbers.extend(find_number_indeces(line));
        }

        for (star_index, _) in star_indeces {
            sum += find_prodcut(&numbers, star_index);
        }

        sum
    }

    fn find_number_indeces(line: &str) -> Vec<(usize, &str)> {
        let mut found = vec![];
        let numbers = line.split(|ch: char| !ch.is_numeric()).filter(|x| x != &"");
        let mut start_index = 0;

        for number in numbers {
            let index = line[start_index..].find(number).unwrap() + start_index;
            found.push((index, number));
            start_index = index + number.len();
        }

        found
    }

    fn find_prodcut(numbers: &[(usize, &str)], star_index: usize) -> u32 {
        let adjacent_nums: Vec<&(usize, &str)> = numbers
            .iter()
            .filter(|(i, num)| star_index <= i + num.len() && star_index >= i - 1)
            .collect();

        if adjacent_nums.len() == 2 {
            return adjacent_nums
                .iter()
                .map(|(_, num)| num.parse::<u32>().unwrap())
                .product();
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part_1_test() {
        assert_eq!(
            part_1::solve(
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
    fn part_1_adjacent_numbers_test() {
        assert_eq!(
            part_1::solve(
                "467..114..\n\
                .12*......\n\
                ..35..633."
            ),
            514
        );
    }

    #[test]
    fn part_1_negative_numbers_test() {
        assert_eq!(
            part_1::solve(
                "467..114..\n\
                -12*......\n\
                ..35..633."
            ),
            514
        );
    }

    #[test]
    fn part_1_end_test() {
        assert_eq!(
            part_1::solve(
                "467..114..\n\
                -12*.....-\n\
                ..35...633"
            ),
            1147
        );
    }

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(
            part_2::solve(
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
            467835
        );
    }
}
