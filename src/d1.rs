use std::fs;
use std::path::PathBuf;

pub fn part1(file_path: String) -> u32 {
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));

    let mut numbers = Vec::<u32>::new();

    fs::read_to_string(path.as_path())
        .unwrap()
        .lines()
        .map(String::from)
        .into_iter()
        .for_each(|elem| {
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;
            elem.chars().for_each(|c| match c {
                '0'..='9' => {
                    if first.is_none() {
                        first = c.to_digit(10);
                    }
                    last = c.to_digit(10);
                }
                _ => (),
            });
            numbers.push(first.unwrap() * 10 + last.unwrap());
        });

    let mut sum: u32 = 0u32;
    numbers.into_iter().for_each(|elem| {
        sum += elem;
    });

    return sum;
}

const NUMBERS: &'static [&'static str] = &[
    "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
    "eight", "8", "nine", "9",
];

fn str_to_number(input: &str) -> u32 {
    match input {
        "one" | "1" => 1u32,
        "two" | "2" => 2u32,
        "three" | "3" => 3u32,
        "four" | "4" => 4u32,
        "five" | "5" => 5u32,
        "six" | "6" => 6u32,
        "seven" | "7" => 7u32,
        "eight" | "8" => 8u32,
        "nine" | "9" => 9u32,
        _ => 0u32,
    }
}
pub fn part2(file_path: String) -> u32 {
    let mut path: PathBuf = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));

    let mut results: Vec<u32> = Vec::<u32>::new();
    fs::read_to_string(path.as_path())
        .unwrap()
        .lines()
        .map(String::from)
        .into_iter()
        .for_each(|elem| {
            let mut name_matches: Vec<_> = Vec::<(usize, &str)>::new();
            NUMBERS.into_iter().for_each(|number| {
                elem.match_indices(number).for_each(|item| {
                    name_matches.push(item);
                });
            });
            name_matches.sort_by_key(|key| key.0);
            results.push(
                match name_matches.first() {
                    Some(r) => str_to_number(r.1) * 10,
                    None => 0,
                } + match name_matches.last() {
                    Some(r) => str_to_number(r.1),
                    None => 0,
                },
            );
        });
    let mut sum = 0u32;
    results.into_iter().for_each(|r| {
        sum += r;
    });
    return sum;
}

#[cfg(test)]
mod test_d1 {
    #[test]
    pub fn test_d1_p1() {
        assert_eq!(super::part1(String::from("data/d1/test_p1.txt")), 142);
    }
    #[test]
    pub fn test_d1_p2() {
        assert_eq!(super::part2(String::from("data/d1/test_p2.txt")), 281);
    }
}
