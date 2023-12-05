#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("./day05/input.txt").expect("Input file could not be read.");
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

mod utility {
    struct Conversion {
        start: i64,
        end: i64,
        conversion: i64,
    }

    pub(crate) struct Map {
        conversions: Vec<Conversion>,
    }

    impl Map {
        pub(crate) fn new(map: &str) -> Map {
            let mut parsed_map = vec![];
            let mut lines = map.lines();
            lines.next();

            for line in lines {
                let parsed_line: Vec<i64> = line
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect();
                let start = parsed_line[1];
                let end = start + parsed_line[2];
                let conversion = parsed_line[0] - start;
                parsed_map.push(Conversion {
                    start,
                    end,
                    conversion,
                });
            }
            Map {
                conversions: parsed_map,
            }
        }

        pub(crate) fn convert(&self, source: i64) -> i64 {
            let mut conversion = self
                .conversions
                .iter()
                .filter(|x| source >= x.start && source <= x.end);
            if let Some(c) = conversion.next() {
                return source + c.conversion;
            }
            source
        }
    }
}

mod part_1 {
    use crate::utility::Map;

    pub(crate) fn solve(input: &str) -> i64 {
        let mut maps = input.split("\n\n");
        let stripped_line = maps.next().unwrap().replace("seeds: ", "");
        let mut sources: Vec<i64> = stripped_line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        for map in maps {
            let map = Map::new(map);
            let mut destinations = vec![];
            for source in sources {
                destinations.push(map.convert(source));
            }
            sources = destinations;
        }

        *sources.iter().min().unwrap()
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
        assert_eq!(part_2::solve(EXAMPLE_INPUT), 46);
    }
}
