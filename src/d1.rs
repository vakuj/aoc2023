use std::fs;
use std::path::PathBuf;

pub fn part1(file_path: &str) -> u32 {
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

#[cfg(test)]
mod test_d1 {
    use super::part1;
    #[test]
    pub fn test_d1_p1() {
        assert_eq!(part1("data/test_d1p1.txt"), 142);
    }
}
