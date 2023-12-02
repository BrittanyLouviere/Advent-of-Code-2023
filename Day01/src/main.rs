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
            &"1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet"
                .to_string()
        ),
        142
    );
}

fn solve_part_1(input: &str) -> u32 {
    let mut first_num = ' ';
    let mut last_num = ' ';
    let mut result = 0;
    for a in input.chars() {
        if a == '\n' {
            let number = format!("{first_num}{last_num}");
            result += number.parse::<u32>().unwrap();
            first_num = ' ';
            last_num = ' ';
        }
        if a.is_numeric() {
            last_num = a;
            if first_num == ' ' {
                first_num = a;
            }
        }
    }

    let number = format!("{first_num}{last_num}");
    result += number.parse::<u32>().unwrap();

    result
}

#[test]
fn example_input_part_2_test() {
    assert_eq!(
        solve_part_2(
            &"two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen"
                .to_string()
        ),
        281
    );
}

fn solve_part_2(input: &str) -> u32 {
    let text_numbers: HashMap<&str, &str> = HashMap::from([
        ("one", "o1ne"),
        ("two", "t2wo"),
        ("three", "t3hree"),
        ("four", "f4our"),
        ("five", "f5ive"),
        ("six", "s6ix"),
        ("seven", "s7even"),
        ("eight", "e8ight"),
        ("nine", "n9ine"),
    ]);

    let mut formatted_input = input.to_owned();
    for (text, number) in text_numbers {
        formatted_input = formatted_input.replace(text, number);
    }

    solve_part_1(&formatted_input)
}
