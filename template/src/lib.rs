pub fn get_input() -> String {
    std::fs::read_to_string("./{{project-name}}/input.txt").unwrap_or_else(|_| {
        std::fs::read_to_string("../{{project-name}}/input.txt")
            .expect("Input file could not be read.")
    })
}

pub mod part_1 {
    pub fn solve(input: &str) -> u32 {
        todo!()
    }
}

pub mod part_2 {
    pub fn solve(input: &str) -> u32 {
        todo!()
    }
}
