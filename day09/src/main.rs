#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("./day09/input.txt").expect("Input file could not be read.");
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

mod part_1 {
    pub(crate) fn solve(input: &str) -> i32 {
        let mut sum = 0;
        for line in input.lines() {
            let history = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
            sum += extrapolate(history);
        }
        sum
    }

    fn extrapolate(history: Vec<i32>) -> i32 {
        let mut sequences = vec![history];
        loop {
            let current_seq = sequences.last().unwrap();
            let mut new_seq = vec![];
            for i in 0..(current_seq.len()-1) {
                let v1 = current_seq[i];
                let v2 = current_seq[i+1];
                new_seq.push(v2-v1);
            }
            if new_seq.iter().all(|x| x == &0) {
                break;
            }
            sequences.push(new_seq);
        }

        let mut current_num = 0;
        for seq in sequences.iter().rev(){
            current_num += seq.last().unwrap();
        }
        current_num
    }
}

mod part_2 {
    pub(crate) fn solve(input: &str) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn example_input_part_1_test() {
        assert_eq!(part_1::solve(EXAMPLE_INPUT), 114);
    }

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(part_2::solve(EXAMPLE_INPUT), 0);
    }
}
