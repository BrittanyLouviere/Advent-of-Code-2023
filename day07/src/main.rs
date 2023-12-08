#![warn(clippy::pedantic)]
use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("./day07/input.txt").expect("Input file could not be read.");
    println!("Part 1: {}", part_1::solve(&input));
    println!("Part 2: {}", part_2::solve(&input));
}

pub(crate) mod utility {
    use std::collections::{HashMap, HashSet};

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
        pub(crate) cards: Vec<u64>,
        pub(crate) bid: u64,
    }

    impl Hand {
        pub(crate) fn new(
            cards: &str,
            bid: u64,
            card_ranks: &HashMap<char, u64>,
            jokers_enabled: bool,
        ) -> Hand {
            let hand_type = Self::get_type(cards, jokers_enabled);
            let cards = Self::convert_cards_to_ranks(cards, card_ranks);
            Hand {
                hand_type,
                cards,
                bid,
            }
        }

        fn convert_cards_to_ranks(cards: &str, card_ranks: &HashMap<char, u64>) -> Vec<u64> {
            let mut ranks = vec![];
            for c in cards.chars() {
                let test = card_ranks[&c];
                ranks.push(test);
            }
            ranks
        }

        fn get_type(cards: &str, jokers_enabled: bool) -> HandType {
            let mut unique_chars = HashSet::new();
            for c in cards.chars() {
                unique_chars.insert(c);
            }
            let mut char_counts = vec![];
            for i in unique_chars.drain() {
                let count = cards.chars().filter(|x| x == &i).count();
                char_counts.push(count);
            }
            char_counts.sort_unstable();
            let joker_count = if jokers_enabled {
                cards.chars().filter(|x| x == &'J').count()
            } else {
                0
            };

            match (&char_counts[..], joker_count) {
                ([5], _) | ([1, 4], 1 | 4) | ([2, 3], 2 | 3) => HandType::FiveKind,
                ([1, 4], 0) | ([1, 1, 3], 1 | 3) | ([1, 2, 2], 2) => HandType::FourKind,
                ([2, 3], 0) | ([1, 2, 2], 1) => HandType::FullHouse,
                ([1, 1, 3], 0) | ([1, 1, 1, 2], 1 | 2) => HandType::ThreeKind,
                ([1, 2, 2], 0) => HandType::TwoPair,
                ([1, 1, 1, 2], 0) | ([1, 1, 1, 1, 1], 1) => HandType::OnePair,
                _ => HandType::HighCard,
            }
        }
    }
}

mod part_1 {
    use crate::utility::Hand;
    use std::{collections::HashMap, vec};

    pub(crate) fn solve(input: &str) -> u64 {
        let card_ranks: HashMap<_, _> = "23456789TJQKA"
            .chars()
            .zip([2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14])
            .collect();
        let mut hands = vec![];

        for line in input.lines() {
            let mut parsed = line.split_whitespace();
            let cards = parsed.next().unwrap().to_string();
            let bid = parsed.next().unwrap().parse::<u64>().unwrap();

            hands.push(Hand::new(&cards, bid, &card_ranks, false));
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
    use crate::utility::Hand;
    use std::{collections::HashMap, vec};

    pub(crate) fn solve(input: &str) -> u64 {
        let card_ranks: HashMap<_, _> = "J23456789TQKA"
            .chars()
            .zip([2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14])
            .collect();
        let mut hands = vec![];

        for line in input.lines() {
            let mut parsed = line.split_whitespace();
            let cards = parsed.next().unwrap().to_string();
            let bid = parsed.next().unwrap().parse::<u64>().unwrap();

            hands.push(Hand::new(&cards, bid, &card_ranks, true));
        }

        hands.sort();
        let mut sum = 0;
        for (i, hand) in hands.iter().enumerate() {
            sum += (i as u64 + 1) * hand.bid;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
        let card_ranks: HashMap<_, _> = "23456789TJQKA"
            .chars()
            .zip([2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14])
            .collect();
        let mut hands = vec![];
        for cards in ["32T3K", "KK677", "KTJJT", "T55J5", "QQQJA"] {
            hands.push(Hand::new(cards, 0, &card_ranks, false));
        }

        assert_eq!(hands[0].hand_type, HandType::OnePair);
        assert_eq!(hands[1].hand_type, HandType::TwoPair);
        assert_eq!(hands[2].hand_type, HandType::TwoPair);
        assert_eq!(hands[3].hand_type, HandType::ThreeKind);
        assert_eq!(hands[4].hand_type, HandType::ThreeKind);
    }

    #[test]
    fn example_input_part_2_test() {
        assert_eq!(part_2::solve(EXAMPLE_INPUT), 5905);
    }
}
