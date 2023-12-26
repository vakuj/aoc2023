use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Card {
    nr: i32,
    correct: Vec<i32>,
}

impl Card {
    pub fn new() -> Self {
        Card {
            nr: 0,
            correct: Vec::<i32>::new(),
        }
    }

    pub fn from_string(line: &str) -> Self {
        let mut card = Card::new();
        let (card_str, line) = line.split_once(':').expect("Colon seperator not found");

        let (_, card_nr) = card_str.split_once(" ").unwrap();
        card.nr = card_nr.trim().parse::<i32>().unwrap();

        let (win_str, nbr_str) = line.split_once('|').expect("| seperator not found");

        let mut winning = HashSet::<i32>::new();

        win_str.trim().split(' ').into_iter().for_each(|nbr| {
            if let Ok(winning_nbr) = nbr.trim().parse::<i32>() {
                let _ = winning.insert(winning_nbr);
            }
        });

        nbr_str.trim().split(' ').into_iter().for_each(|nbr| {
            if let Ok(draw) = nbr.parse::<i32>() {
                if winning.contains(&draw) {
                    card.correct.push(draw);
                }
            }
        });

        return card;
    }
}

pub fn part1(file_path: String) -> u32 {
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));

    let mut cards = Vec::<Card>::new();
    fs::read_to_string(path.as_path())
        .expect("Could not open file")
        .lines()
        .for_each(|line| cards.push(Card::from_string(line)));

    let mut sum = 0u32;

    cards.into_iter().for_each(|card| {
        if card.correct.len() >= 1 {
            sum += 1 << (card.correct.len() - 1);
        }
        println!("{:?}", card);
    });

    return sum;
}

pub fn part2(file_path: String) -> u32 {
    0
}

#[cfg(test)]
mod test_d4 {

    #[test]
    pub fn test_d4_p1() {
        let a = super::part1(String::from("data/d4/test_p1.txt"));
        assert_eq!(a, 13);
    }
}
