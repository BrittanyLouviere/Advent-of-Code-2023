#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("./day05/input.txt").expect("Input file could not be read.");
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

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13\n\
    \n\
    seed-to-soil map:\n\
    50 98 2\n\
    52 50 48\n\
    \n\
    soil-to-fertilizer map:\n\
    0 15 37\n\
    37 52 2\n\
    39 0 15\n\
    \n\
    fertilizer-to-water map:\n\
    49 53 8\n\
    0 11 42\n\
    42 0 7\n\
    57 7 4\n\
    \n\
    water-to-light map:\n\
    88 18 7\n\
    18 25 70\n\
    \n\
    light-to-temperature map:\n\
    45 77 23\n\
    81 45 19\n\
    68 64 13\n\
    \n\
    temperature-to-humidity map:\n\
    0 69 1\n\
    1 0 69\n\
    \n\
    humidity-to-location map:\n\
    60 56 37\n\
    56 93 4";

    #[test]
    fn example_input_part_1_test() {
        assert_eq!(part_1::solve(EXAMPLE_INPUT), 35);
    }

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(part_2::solve(EXAMPLE_INPUT), 0);
    }
}
