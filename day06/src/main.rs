#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("./day06/input.txt").expect("Input file could not be read.");
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

pub(crate) mod utility {
    pub(crate) fn find_win_count(time: u64, dist_to_beat: u64) -> u64 {
        let mut win_count = 0;
        for charge_time in 1..time {
            if charge_time * (time - charge_time) > dist_to_beat {
                win_count += 1;
            }
        }
        win_count
    }
}

mod part_1 {
    use crate::utility::find_win_count;
    pub(crate) fn solve(input: &str) -> u64 {
        let lines: Vec<Vec<&str>> = input
            .lines()
            .map(|x| x.split_whitespace().collect())
            .collect();

        let times = &lines[0][1..];
        let distances = &lines[1][1..];
        let mut win_prod = 1;

        for i in 0..times.len() {
            let time = times[i].parse::<u64>().unwrap();
            let distance = distances[i].parse::<u64>().unwrap();

            win_prod *= find_win_count(time, distance);
        }
        win_prod
    }
}

mod part_2 {
    use crate::utility::find_win_count;
    pub(crate) fn solve(input: &str) -> u64 {
        let lines: Vec<&str> = input.lines().collect();

        let time = &lines[0]
            .replace("Time:", "")
            .replace(' ', "")
            .parse::<u64>()
            .unwrap();
        let distance = &lines[1]
            .replace("Distance:", "")
            .replace(' ', "")
            .parse::<u64>()
            .unwrap();

        find_win_count(*time, *distance)
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
        assert_eq!(part_2::solve(EXAMPLE_INPUT), 71503);
    }
}
