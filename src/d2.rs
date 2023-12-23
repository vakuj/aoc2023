use std::fs;
use std::path::PathBuf;

const BAG: Set = Set {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug, Clone)]
pub struct Game {
    sets: Vec<Set>,
    number: u32,
    total: Set,
    max: Set,
}

impl Game {
    pub fn new(number: u32) -> Self {
        Game {
            sets: Vec::<Set>::new(),
            number: number,
            total: Set::new(),
            max: Set::new(),
        }
    }

    pub fn get_number(&self) -> u32 {
        self.number
    }
    pub fn get_total(&self) -> Set {
        self.total
    }

    pub fn get_power(&self) -> u32 {
        self.max.red * self.max.green * self.max.blue
    }

    pub fn parse(game_str: &str) -> Option<Self> {
        let (game_str, sets) = game_str.trim().split_once(':').unwrap();
        let (_, number) = game_str.trim().split_once(' ').unwrap();
        let mut game = Game::new(number.parse().unwrap());
        sets.split(';')
            .for_each(|set_str| game.add_set(Set::parse(set_str)));
        Some(game)
    }

    pub fn add_set(&mut self, set: Set) {
        self.total.insert(set);
        self.max.swap_max(set);
        self.sets.push(set);
    }

    pub fn all_sets_contained(&self, limit: Set) -> bool {
        for set in &self.sets {
            if !set.is_contained(limit) {
                return false;
            }
        }
        true
    }

    pub fn to_string(&self) -> String {
        String::from(format!(
            "Game number: {:?}\n Total: {}\n   Max: {}",
            self.number,
            self.total.to_string(),
            self.max.to_string()
        ))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    pub fn new() -> Self {
        Set {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    pub fn parse(set_str: &str) -> Self {
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;

        set_str.split(',').for_each(|color_str| {
            let pair_opt = color_str.trim().split_once(' ');
            match pair_opt {
                Some(pair) => match pair.1 {
                    "red" => r += pair.0.parse::<u32>().unwrap(),
                    "green" => g += pair.0.parse::<u32>().unwrap(),
                    "blue" => b += pair.0.parse::<u32>().unwrap(),
                    _ => println!("{:?} : {:?}", pair.0, pair.1),
                },
                None => println!("Set parsed as non"),
            };
        });

        Set {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn swap_max(&mut self, other: Set) {
        if self.red == 0 || self.red < other.red {
            self.red = other.red;
        }
        if self.green == 0 || self.green < other.green {
            self.green = other.green;
        }
        if self.blue == 0 || self.blue < other.blue {
            self.blue = other.blue;
        }
    }

    pub fn insert(&mut self, other: Set) {
        self.red += other.red;
        self.green += other.green;
        self.blue += other.blue;
    }
    pub fn is_contained(&self, limit: Set) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }
    pub fn to_string(&self) -> String {
        String::from(format!(
            "(R, G, B): ({}, {}, {})",
            self.red, self.green, self.blue
        ))
    }
}

pub fn both_parts(file_path: String) -> (u32, u32) {
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));

    let mut games: Vec<Game> = Vec::<Game>::new();

    fs::read_to_string(path.as_path())
        .unwrap()
        .lines()
        .for_each(|line| {
            if let Some(game) = Game::parse(line) {
                games.push(game)
            }
        });
    let mut part1 = 0u32;
    let mut part2 = 0u32;
    games.into_iter().for_each(|game| {
        if game.all_sets_contained(BAG) {
            part1 += game.get_number();
        }
        part2 += game.get_power();
    });
    return (part1, part2);
}

#[cfg(test)]
mod test_d1 {

    #[test]
    pub fn test_d2_p1() {
        let (a, _) = super::both_parts(String::from("data/d2/test_p1.txt"));
        assert_eq!(a, 8);
    }

    #[test]
    pub fn test_d2_p2() {
        let (_, a) = super::both_parts(String::from("data/d2/test_p2.txt"));
        assert_eq!(a, 2286);
    }
}
