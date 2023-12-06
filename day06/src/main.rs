#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("./day06/input.txt").expect("Input file could not be read.");
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

mod part_1 {
    pub(crate) fn solve(input: &str) -> u32 {
        let lines: Vec<Vec<&str>> = input
            .lines()
            .map(|x| x.split_whitespace().collect())
            .collect();

        let times = &lines[0][1..];
        let distances = &lines[1][1..];
        let mut win_prod = 1;

        for i in 0..times.len() {
            let time = times[i].parse::<i32>().unwrap();
            let distance = distances[i].parse::<i32>().unwrap();
            let mut win_count = 0;

            for charge_time in 1..time {
                if charge_time * (time - charge_time) > distance {
                    win_count += 1;
                }
            }
            win_prod *= win_count;
        }
        win_prod
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

    const EXAMPLE_INPUT: &str = "Time:      7  15   30\n\
    Distance:  9  40  200";

    #[test]
    fn example_input_part_1_test() {
        assert_eq!(part_1::solve(EXAMPLE_INPUT), 288);
    }

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(part_2::solve(EXAMPLE_INPUT), 0);
    }
}
