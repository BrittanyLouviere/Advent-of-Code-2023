use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

#[test]
fn example_input_part_1_test() {
    assert_eq!(solve_part_1(&"".to_string()), 0);
}

fn solve_part_1(input: &String) -> u32 {
    0
}

#[test]
fn example_input_part_2_test() {
    assert_eq!(solve_part_2(&"".to_string()), 0);
}

fn solve_part_2(input: &String) -> u32 {
    0
}
