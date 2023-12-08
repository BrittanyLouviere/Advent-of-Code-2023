#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("./day08/input.txt").expect("Input file could not be read.");
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

pub(crate) mod utility {
    use std::collections::HashMap;

    pub(crate) fn create_map(lines: Vec<&str>) -> HashMap<String, (String, String)> {
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        for line in lines {
            let parsed_line: Vec<_> = line.split_whitespace().collect();
            let name = parsed_line[0].to_string();
            let left = parsed_line[2][1..4].to_string();
            let right = parsed_line[3][..3].to_string();
            map.insert(name, (left, right));
        }
        map
    }
}

mod part_1 {
    use crate::utility::create_map;

    pub(crate) fn solve(input: &str) -> u32 {
        let mut lines = input.lines();
        let path: Vec<_> = lines.next().unwrap().chars().collect();
        lines.next();
        let lines: Vec<&str> = lines.collect();

        let map = create_map(lines);

        let mut current_node = "AAA";
        let mut current_dir = 0;
        let mut step_count = 0;
        while current_node != "ZZZ" {
            let next_node = &map[current_node];
            current_node = if path[current_dir] == 'L' {
                &next_node.0
            } else {
                &next_node.1
            };
            current_dir += 1;
            current_dir %= path.len();
            step_count += 1;
        }
        step_count
    }
}

mod part_2 {
    use num::integer::lcm;
    use std::collections::HashMap;

    use crate::utility::create_map;

    pub(crate) fn solve(input: &str) -> u64 {
        let mut lines = input.lines();
        let path: Vec<_> = lines.next().unwrap().chars().collect();
        lines.next();
        let lines: Vec<&str> = lines.collect();

        let map = create_map(lines);

        let start_nodes = map
            .keys()
            .filter(|x| x.ends_with('A'))
            .map(std::string::ToString::to_string);

        let mut node_patterns: Vec<u64> = vec![];
        for node in start_nodes {
            node_patterns.push(get_node_pattern(&map, &node, &path));
        }

        let mut steps: u64 = 1;
        for x in &node_patterns {
            steps = lcm(steps, *x);
        }
        steps
    }

    fn get_node_pattern(
        map: &HashMap<String, (String, String)>,
        node: &str,
        path: &Vec<char>,
    ) -> u64 {
        let mut current_node = node;
        let mut step_count = 0;
        let mut current_dir = 0;

        loop {
            let next_node = &map[current_node];
            current_node = if path[current_dir] == 'L' {
                &next_node.0
            } else {
                &next_node.1
            };
            current_dir += 1;
            current_dir %= path.len();
            step_count += 1;

            if current_node.ends_with('Z') {
                break;
            }
        }
        step_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_INPUT_2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn example_input_part_1_test() {
        assert_eq!(part_1::solve(EXAMPLE_INPUT), 2);
        assert_eq!(part_1::solve(EXAMPLE_INPUT_2), 6);
    }

    const EXAMPLE_INPUT_3: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(part_2::solve(EXAMPLE_INPUT_3), 6);
    }
}
