#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("./day05/input.txt").expect("Input file could not be read.");
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

mod utility {
    use std::ops::RangeInclusive;

    struct Conversion {
        range: RangeInclusive<i64>,
        conversion: i64,
    }

    impl Conversion {
        fn contains(&self, item: i64) -> bool {
            self.range.contains(&item)
        }
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
                let end = start + parsed_line[2] - 1;
                let conversion = parsed_line[0] - start;
                parsed_map.push(Conversion {
                    range: start..=end,
                    conversion,
                });
            }
            Map {
                conversions: parsed_map,
            }
        }

        pub(crate) fn convert(&self, source: i64) -> i64 {
            let mut conversion = self.conversions.iter().filter(|x| x.contains(source));
            if let Some(c) = conversion.next() {
                return source + c.conversion;
            }
            source
        }

        pub(crate) fn convert_range(
            &self,
            source_range: RangeInclusive<i64>,
        ) -> Vec<RangeInclusive<i64>> {
            let mut dest_range = vec![];
            let mut source_ranges = vec![source_range];

            'source_loop: while let Some(source) = source_ranges.pop() {
                for conv in &self.conversions {
                    let start_contained = conv.contains(*source.start());
                    let end_contained = conv.contains(*source.end());

                    if start_contained && end_contained {
                        // fully mapped by this conversion
                        let new_start = source.start() + conv.conversion;
                        let new_end = source.end() + conv.conversion;
                        dest_range.push(new_start..=new_end);

                        continue 'source_loop;
                    } else if start_contained {
                        // first half mapped
                        let new_start = source.start() + conv.conversion;
                        let new_end = conv.range.end() + conv.conversion;
                        dest_range.push(new_start..=new_end);

                        // second half not mapped
                        let new_start = conv.range.end() + 1;
                        let new_end = source.end();
                        source_ranges.push(new_start..=*new_end);

                        continue 'source_loop;
                    } else if end_contained {
                        // second half mapped
                        let new_start = conv.range.start() + conv.conversion;
                        let new_end = source.end() + conv.conversion;
                        dest_range.push(new_start..=new_end);

                        // first half not mapped
                        let new_start = source.start();
                        let new_end = conv.range.start() - 1;
                        source_ranges.push(*new_start..=new_end);

                        continue 'source_loop;
                    }
                }

                // no mapping to any conversion
                dest_range.push(source.clone());
            }

            dest_range
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
    use crate::utility::Map;
    use std::ops::RangeInclusive;

    pub(crate) fn solve(input: &str) -> i64 {
        let mut maps = input.split("\n\n");
        let stripped_line = maps.next().unwrap().replace("seeds: ", "");
        let mut seeds = stripped_line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap());

        let mut sources: Vec<RangeInclusive<i64>> = [].to_vec();
        while let (Some(start), Some(range)) = (seeds.next(), seeds.next()) {
            sources.push(start..=(start + range));
        }

        for map in maps {
            let map = Map::new(map);
            let mut destinations = vec![];
            for source in sources {
                destinations.append(&mut map.convert_range(source));
            }
            sources = destinations;
        }

        *sources
            .iter()
            .map(std::ops::RangeInclusive::start)
            .min()
            .unwrap()
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
