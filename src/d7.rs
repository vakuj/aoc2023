use core::panic;
use std::cmp::Ordering;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<u32>,
    bid: u32,
    rank: Rank,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Rank {
    Undefined = 0,
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfKind = 4,
    FullHouse = 5,
    FourOfKind = 6,
    FiveOfKind = 7,
}
impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Vec::<u32>::new(),
            bid: 0,
            rank: Rank::Undefined,
        }
    }

    pub fn from_string(line: &str) -> Option<Self> {
        if let Some((a, b)) = line.trim().split_once(' ') {
            let mut hand = Hand::new();
            let mut uhand = Vec::<(u32, u32)>::new();
            a.chars().for_each(|c| {
                let value: u32 = match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    '9' => 9,
                    '8' => 8,
                    '7' => 7,
                    '6' => 6,
                    '5' => 5,
                    '4' => 4,
                    '3' => 3,
                    '2' => 2,
                    _ => panic!("Could not parse {c} as a card"),
                };

                hand.cards.push(value);
                let mut inserted = false;
                for ii in 0..uhand.len() {
                    if uhand[ii].0 == value {
                        uhand[ii].1 += 1;
                        inserted = true;
                        break;
                    }
                }
                if !inserted {
                    uhand.push((value, 1));
                }
            });
            match uhand.len() {
                1 => hand.rank = Rank::FiveOfKind,
                2 => {
                    if uhand[0].1 == 4 || uhand[1].1 == 4 {
                        hand.rank = Rank::FourOfKind;
                    } else {
                        hand.rank = Rank::FullHouse;
                    }
                }
                3 => {
                    // 2x2 + 1 or  1x3+1+1
                    if uhand[0].1 == 3 || uhand[1].1 == 3 || uhand[2].1 == 3 {
                        hand.rank = Rank::ThreeOfKind;
                    } else {
                        hand.rank = Rank::TwoPair;
                    }
                }
                4 => {
                    // 1x2+1+1+1
                    hand.rank = Rank::OnePair;
                }
                5 => {
                    // 1+1+1+1+1
                    hand.rank = Rank::HighCard;
                }
                _ => {
                    // None where set
                    hand.rank = Rank::Undefined;
                }
            }
            match b.parse() {
                Ok(bid) => hand.bid = bid,
                Err(err) => panic!("Could not parse {b} as bid: {err}"),
            }
            return Some(hand);
        }
        None
    }

    pub fn cmp(&self, other: &Hand) -> Ordering {
        if self.rank > other.rank {
            return Ordering::Greater;
        }
        if self.rank < other.rank {
            return Ordering::Less;
        }
        for ii in 0..5 {
            if self.cards[ii] == other.cards[ii] {
                continue;
            }
            if self.cards[ii] > other.cards[ii] {
                return Ordering::Greater;
            }
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

pub fn load_data(file_path: String) -> Result<Vec<Hand>, String> {
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));
    let binding = fs::read_to_string(path).expect("Should load");
    let lines = binding.lines().collect::<Vec<&str>>();
    let mut hands = Vec::<Hand>::new();
    lines.into_iter().for_each(|line| {
        if let Some(hand) = Hand::from_string(line) {
            hands.push(hand)
        }
    });
    // hands.iter().for_each(|hand| println!("{hand:?}"));
    Ok(hands)
}

pub fn part1(file_path: String) -> u32 {
    match load_data(file_path) {
        Ok(mut hands) => {
            hands.sort_by(|a, b| a.cmp(b));
            return hands
                .iter()
                .enumerate()
                .fold(0u32, |acc, (ii, hand)| acc + hand.bid * (ii as u32 + 1));
        }
        Err(str) => panic!("{str}"),
    }
}

pub fn part2(file_path: String) -> usize {
    0
}

#[cfg(test)]
mod test_d7 {
    use crate::d7::{load_data, Rank};

    #[test]
    pub fn load_test_d7_p1() {
        match load_data(String::from("data/d7/test_p1.txt")) {
            Ok(data) => {
                assert_eq!(data[0].bid, 765);
                assert_eq!(data[1].bid, 684);
                assert_eq!(data[2].bid, 28);
                assert_eq!(data[3].bid, 220);
                assert_eq!(data[4].bid, 483);

                assert_eq!(data[0].rank, Rank::OnePair);
                assert_eq!(data[1].rank, Rank::ThreeOfKind);
                assert_eq!(data[2].rank, Rank::TwoPair);
                assert_eq!(data[3].rank, Rank::TwoPair);
                assert_eq!(data[4].rank, Rank::ThreeOfKind);

                assert_eq!(data[0].cards[0], 3);
                assert_eq!(data[0].cards[1], 2);
                assert_eq!(data[0].cards[2], 10);
                assert_eq!(data[0].cards[3], 3);
                assert_eq!(data[0].cards[4], 13);

                assert_eq!(data[1].cards[0], 10);
                assert_eq!(data[2].cards[0], 13);
                assert_eq!(data[3].cards[0], 13);
                assert_eq!(data[4].cards[0], 12);
            }
            Err(str) => panic!("{str}"),
        }
    }

    #[test]
    pub fn test_d7_p1() {
        let a = super::part1(String::from("data/d7/test_p1.txt"));
        assert_eq!(a, 6440);
    }
    #[test]
    pub fn test_d7_p1real() {
        let a = super::part1(String::from("data/d7/input.txt"));
        assert_eq!(a, 0);
    }
}
