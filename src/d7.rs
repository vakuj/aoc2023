use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Hand {
    cards: &'static [Option<Card>; 5],
    bid: u32,
}
#[derive(Debug)]
pub struct Card {
    value: u32,
    count: u32,
}
impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: &[None, None, None, None, None],
            bid: 0,
        }
    }

    pub fn from_string(line: String) -> Option<Self> {
        todo!()
    }

    pub fn contains(self, other: &Card) -> bool {
        todo!()
    }
}

impl Card {
    pub fn new() -> Self {
        Card { value: 0, count: 0 }
    }

    pub fn from_char(c: char) -> Option<Self> {
        todo!()
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
