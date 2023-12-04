#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./day04/input.txt").unwrap();
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

mod part_1 {
    pub(crate) fn solve(input: &str) -> u32 {
        let mut points = 0;
        for line in input.lines() {
            let start = line.find(':').unwrap() + 1;
            let mid = line.find('|').unwrap();

            let win_nums: Vec<&str> = line[start..(mid - 1)].split_whitespace().collect();
            let my_nums = line[(mid + 2)..].split_whitespace();

            let count = my_nums.filter(|x| win_nums.contains(x)).count();
            if count > 0 {
                points += 2u32.pow((count - 1).try_into().unwrap());
            }
        }
        points
    }
}

mod part_2 {
    pub(crate) fn solve(input: &str) -> u32 {
        let line_count = input.matches('\n').count();
        let mut card_counts = vec![1; line_count + 1];

        for (card_id, line) in input.lines().enumerate() {
            let start = line.find(':').unwrap() + 1;
            let mid = line.find('|').unwrap();
            let win_nums: Vec<&str> = line[start..(mid - 1)].split_whitespace().collect();
            let my_nums = line[(mid + 2)..].split_whitespace();

            let count = my_nums.filter(|x| win_nums.contains(x)).count();

            for i in (card_id + 1)..=(card_id + count) {
                card_counts[i] += card_counts[card_id];
            }
        }
        card_counts.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example_input_part_1_test() {
        assert_eq!(part_1::solve(&EXAMPLE_INPUT), 13);
    }

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(part_2::solve(&EXAMPLE_INPUT), 30);
    }
}
