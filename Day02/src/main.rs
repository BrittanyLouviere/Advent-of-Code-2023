#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs};

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

#[test]
fn example_input_part_1_test() {
    assert_eq!(
        solve_part_1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        ),
        8
    );
}

fn solve_part_1(input: &str) -> u32 {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let color_counts: HashMap<&str, u32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut result = 0;

    for line in input.lines() {
        let index = line.find(':').unwrap();
        let mut game_id = &line[5..index];

        let split_line: Vec<&str> = line[index + 1..].split([',', ';']).collect();

        for hand in split_line {
            let hand_split: Vec<&str> = hand.split_whitespace().collect();
            let count = hand_split[0].parse::<u32>().unwrap();
            let color = hand_split[1];

            if count > color_counts[color] {
                game_id = "0";
                break;
            }
        }
        result += game_id.parse::<u32>().unwrap();
    }
    result
}

#[test]
fn example_input_part_2_test() {
    assert_eq!(solve_part_2(&"".to_string()), 0);
}

fn solve_part_2(_input: &str) -> u32 {
    0
}
