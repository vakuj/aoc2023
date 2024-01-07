use core::panic;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Hand<'a> {
    cards: &'a mut [Option<Card>; 5],
    bid: u32,
    index: usize,
}
#[derive(Debug)]
pub struct Card {
    value: u32,
    count: u32,
}
impl<'a> Hand<'a> {
    pub fn new() -> Self {
        Hand {
            cards: &mut [None, None, None, None, None],
            bid: 0,
            index: 0,
        }
    }

    pub fn from_string(line: String) -> Option<Self> {
        if let Some((a, b)) = line.trim().split_once(' ') {
            let mut hand = Hand::new();

            a.chars().for_each(|c| {
                match Card::from_char(c) {
                    Some(card) => hand.insert(&card),
                    None => panic!("Could not parse {c} as a card"),
                };
            });

            match b.parse() {
                Ok(bid) => hand.bid = bid,
                Err(err) => panic!("Could not parse {b} as bid: {err}"),
            }
            return Some(hand);
        }
        None
    }

    pub fn contains(self, other: &Card) -> Option<usize> {
        for ii in 0..5 {
            if let Some(card) = self.cards[ii] {
                if card.value == other.value {
                    return Some(ii);
                }
            }
        }
        None
    }

    pub fn insert(&mut self, other: &Card) {
        match self.contains(other) {
            Some(index) => {
                if let Some(card) = &self.cards[index] {
                    card.incr_count();
                }
            }
            None => {
                if self.index >= 5 {
                    panic!(
                        "Out-of-bound insert of card attempted @ index {}",
                        self.index
                    );
                }
                self.cards[self.index] = Some(*other);
                self.index += 1;
            }
        }
    }

    pub fn rank(&self) -> u32 {
        match self.cards {
            [Some(card1), None, None, None, None] => {
                // 1x5
                7
            }
            [Some(card1), Some(card2), None, None, None] => {
                // 1x4+1 or (1x3+1x2)
                if card1.count == 4 || card2.count == 4 {
                    6
                } else {
                    5
                }
            }
            [Some(card1), Some(card2), Some(card3), None, None] => {
                // 2x2 + 1 or  1x3+1+1
                if card1.count == 3 || card2.count == 3 || card3.count == 3 {
                    3
                } else {
                    4
                }
            }
            [Some(card1), Some(card2), Some(card3), Some(card4), None] => {
                // 1x2+1+1+1
                2
            }
            [Some(card1), Some(card2), Some(card3), Some(card4), Some(card5)] => {
                // 1+1+1+1+1
                1
            }
            _ => {
                // None where set
                0
            }
        }
    }
}

impl Card {
    pub fn new(value: u32) -> Self {
        Card { value, count: 0 }
    }

    pub fn from_char(c: char) -> Option<Self> {
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
            _ => 0,
        };
        if value != 0 {
            return Some(Card::new(value));
        }
        None
    }

    pub fn incr_count(&mut self) {
        self.count += 1;
    }
}

pub fn load_data(file_path: String) -> Result<Vec<Hand>, String> {
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));
    let binding = fs::read_to_string(path).expect("Should load");
    let lines = binding.lines().collect::<Vec<&str>>();
    let mut hands = Vec::<Hand>::new();

    Ok(hands)
}

pub fn part1(file_path: String) -> u32 {
    0
}

pub fn part2(file_path: String) -> usize {
    0
}

#[cfg(test)]
mod test_d7 {
    use crate::d7::load_data;

    #[test]
    pub fn load_test_d7_p1() {
        match load_data(String::from("data/d7/test_p1.txt")) {
            Ok(data) => {}
            Err(str) => panic!("{str}"),
        }
    }

    #[test]
    pub fn test_d6_p1() {
        let a = super::part1(String::from("data/d7/test_p1.txt"));
        assert_eq!(a, 6440);
    }
}
