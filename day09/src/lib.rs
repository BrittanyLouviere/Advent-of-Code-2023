pub fn get_input() -> String {
    std::fs::read_to_string("./day09/input.txt").unwrap_or_else(|_| {
        std::fs::read_to_string("../day09/input.txt").expect("Input file could not be read.")
    })
}

pub(crate) mod utility {
    use num::traits::NumAssign;

    pub(crate) fn extrapolate<T>(history: Vec<T>, next_val: bool) -> T
    where
        T: NumAssign + Copy,
    {
        let mut sequences = vec![history];
        loop {
            let current_seq = sequences.last().unwrap();
            let mut new_seq = vec![];
            for i in 0..(current_seq.len() - 1) {
                let v1 = current_seq[i];
                let v2 = current_seq[i + 1];
                new_seq.push(v2 - v1);
            }
            if new_seq.iter().all(|x| x == &T::zero()) {
                break;
            }
            sequences.push(new_seq);
        }

        let mut current_num = T::zero();
        for seq in sequences.iter().rev() {
            if next_val {
                current_num += *seq.last().unwrap();
            } else {
                current_num = *seq.first().unwrap() - current_num;
            }
        }
        current_num
    }
}

pub mod part_1 {
    use crate::utility::extrapolate;
    use std::thread;

    pub fn solve(input: &str) -> i32 {
        let mut handles = vec![];
        for line in input.lines() {
            let history = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            handles.push(thread::spawn(|| extrapolate(history, true)));
        }

        handles.into_iter().map(|x| x.join().unwrap()).sum()
    }
}

pub mod part_2 {
    use crate::utility::extrapolate;
    use std::thread;

    pub fn solve(input: &str) -> i32 {
        let mut handles = vec![];
        for line in input.lines() {
            let history = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            handles.push(thread::spawn(|| extrapolate(history, false)));
        }
        handles.into_iter().map(|x| x.join().unwrap()).sum()
    }
}
