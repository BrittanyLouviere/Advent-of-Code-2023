#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("./day07/input.txt").expect("Input file could not be read.");
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

pub(crate) mod utility {
    use std::collections::HashSet;

    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
    pub(crate) enum HandType {
        FiveKind = 6,
        FourKind = 5,
        FullHouse = 4,
        ThreeKind = 3,
        TwoPair = 2,
        OnePair = 1,
        HighCard = 0,
    }

    #[derive(PartialOrd, PartialEq, Eq, Ord)]
    pub(crate) struct Hand {
        pub(crate) hand_type: HandType,
        pub(crate) cards: String,
        pub(crate) bid: u64,
    }

    //      my custom definition of Ord isn't working?
    // impl Ord for Hand {
    //     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    //         let sh = self
    //             .cards
    //             .replace("A", "Z")
    //             .replace("K", "Y")
    //             .replace("Q", "X")
    //             .replace("J", "W")
    //             .replace("T", "V");
    //         let oh = other
    //             .cards
    //             .replace("A", "Z")
    //             .replace("K", "Y")
    //             .replace("Q", "X")
    //             .replace("J", "W")
    //             .replace("T", "V");
    //         (&self.hand_type, sh).cmp(&(&other.hand_type, oh))
    //     }
    // }

    impl Hand {
        pub(crate) fn new(cards: String, bid: u64) -> Hand {
            let hand_type = Self::get_type(&cards);
            Hand {
                hand_type,
                cards,
                bid,
            }
        }

        fn get_type(cards: &str) -> HandType {
            let mut unique_chars = HashSet::new();
            for c in cards.chars() {
                unique_chars.insert(c);
            }
            let mut char_counts = vec![];
            for i in unique_chars.drain() {
                let count = cards.chars().filter(|x| x == &i).count();
                char_counts.push((i, count));
            }
            let diff_count = char_counts.len();

            match diff_count {
                1 => HandType::FiveKind,
                2 => {
                    if char_counts[0].1 == 4 || char_counts[0].1 == 1 {
                        HandType::FourKind
                    } else {
                        HandType::FullHouse
                    }
                }
                3 => {
                    if char_counts[0].1 == 2 || char_counts[1].1 == 2 {
                        HandType::TwoPair
                    } else {
                        HandType::ThreeKind
                    }
                }
                4 => HandType::OnePair,
                _ => HandType::HighCard,
            }
        }
    }
}

mod part_1 {
    use std::vec;

    use crate::utility::Hand;

    pub(crate) fn solve(input: &str) -> u64 {
        let mut hands = vec![];
        for line in input.lines() {
            let mut parsed = line.split_whitespace();
            let cards = parsed
                .next()
                .unwrap()
                .to_string()
                .replace('A', "Z")
                .replace('K', "Y")
                .replace('Q', "X")
                .replace('J', "W")
                .replace('T', "V");
            let bid = parsed.next().unwrap().parse::<u64>().unwrap();

            hands.push(Hand::new(cards, bid));
        }
        hands.sort();
        let mut sum = 0;
        for (i, hand) in hands.iter().enumerate() {
            sum += (i as u64 + 1) * hand.bid;
        }
        sum
    }
}

mod part_2 {
    pub(crate) fn solve(input: &str) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::{Hand, HandType};

    use super::*;

    const EXAMPLE_INPUT: &str = "32T3K 765\n\
    T55J5 684\n\
    KK677 28\n\
    KTJJT 220\n\
    QQQJA 483";

    #[test]
    fn example_input_part_1_test() {
        assert_eq!(part_1::solve(EXAMPLE_INPUT), 6440);
    }

    #[test]
    fn get_type_test() {
        let mut hands = vec![];
        hands.push(Hand::new("32T3K".to_string(), 0));
        hands.push(Hand::new("KK677".to_string(), 0));
        hands.push(Hand::new("KTJJT".to_string(), 0));
        hands.push(Hand::new("T55J5".to_string(), 0));
        hands.push(Hand::new("QQQJA".to_string(), 0));

        assert_eq!(hands[0].hand_type, HandType::OnePair);
        assert_eq!(hands[1].hand_type, HandType::TwoPair);
        assert_eq!(hands[2].hand_type, HandType::TwoPair);
        assert_eq!(hands[3].hand_type, HandType::ThreeKind);
        assert_eq!(hands[4].hand_type, HandType::ThreeKind);

        // assert!(hands[1] > hands[0]);
        // assert!(hands[2] > hands[1]);
        // assert!(hands[4] > hands[3]);
    }

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(part_2::solve(EXAMPLE_INPUT), 0);
    }
}
