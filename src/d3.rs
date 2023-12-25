use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

// static SYMBOLS: &'static str = &"+-*/=%@$#&";
#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct PartNumber {
    value: i32, // encode as negative number for markers
    loc: Coord,
    len: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}
impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x: x, y: y }
    }
}
#[allow(unused)]
impl PartNumber {
    pub fn is_neighbour(&self, other: &PartNumber) -> bool {
        let xmin = self.loc.x - self.len;
        let xmax = self.loc.x + 1;

        let ymin = self.loc.y - 1;
        let ymax = self.loc.y + 1;

        if (ymin..=ymax).contains(&other.loc.y) {
            return (xmin..=xmax).contains(&other.loc.x);
        }
        return false;
    }
}

pub fn part1(file_path: String) -> u32 {
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));

    let mut parts = Vec::<PartNumber>::new();
    let mut symbols = Vec::<PartNumber>::new();
    let mut y = 0i32;
    fs::read_to_string(path.as_path())
        .unwrap()
        .lines()
        .for_each(|line| {
            let mut val = String::new();
            let mut val_len = 0usize;
            line.char_indices().for_each(|(x, c)| match c {
                '0'..='9' => {
                    // store values for later
                    val.push(c);
                    val_len += 1;
                }
                '.' | '\n' => {
                    // regular part number break, check if any value in temporary to store
                    if val_len > 0 {
                        parts.push(PartNumber {
                            value: val.parse().unwrap(),
                            loc: Coord::new(x as i32 - 1, y as i32),
                            len: val_len as i32,
                        });
                        val_len = 0;
                        val.clear();
                    }
                }
                _ => {
                    // as symbol found, check if previous part number exist to store and store symbol
                    if val_len > 0 {
                        parts.push(PartNumber {
                            value: val.parse().unwrap(),
                            loc: Coord::new(x as i32 - 1, y as i32),
                            len: val_len as i32,
                        });
                        val_len = 0;
                        val.clear();
                    }
                    symbols.push(PartNumber {
                        value: -1,
                        loc: Coord::new(x as i32, y as i32),
                        len: 1,
                    });
                }
            });
            if val_len > 0 {
                parts.push(PartNumber {
                    value: val.parse().unwrap(),
                    loc: Coord::new(line.len() as i32 - 1, y as i32),
                    len: val_len as i32,
                });
                val_len = 0;
                val.clear();
            }
            y += 1;
        });
    let mut unique = HashSet::<PartNumber>::new();
    parts.into_iter().for_each(|part| {
        // println!("{:?}", part);
        symbols.iter().for_each(|symbol| {
            if part.is_neighbour(symbol) {
                unique.insert(part);
            }
        });
    });
    let mut sum = 0u32;
    unique.into_iter().for_each(|upart| {
        sum += upart.value as u32;
    });

    return sum;
}

pub fn part2(file_path: String) -> u32 {
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));

    let mut parts = Vec::<PartNumber>::new();
    let mut symbols = Vec::<PartNumber>::new();
    let mut y = 0i32;
    fs::read_to_string(path.as_path())
        .unwrap()
        .lines()
        .for_each(|line| {
            let mut val = String::new();
            let mut val_len = 0usize;
            line.char_indices().for_each(|(x, c)| match c {
                '0'..='9' => {
                    // store values for later
                    val.push(c);
                    val_len += 1;
                }
                '.' | '\n' => {
                    // regular part number break, check if any value in temporary to store
                    if val_len > 0 {
                        parts.push(PartNumber {
                            value: val.parse().unwrap(),
                            loc: Coord::new(x as i32 - 1, y as i32),
                            len: val_len as i32,
                        });
                        val_len = 0;
                        val.clear();
                    }
                }
                _ => {
                    // as symbol found, check if previous part number exist to store and store symbol
                    if val_len > 0 {
                        parts.push(PartNumber {
                            value: val.parse().unwrap(),
                            loc: Coord::new(x as i32 - 1, y as i32),
                            len: val_len as i32,
                        });
                        val_len = 0;
                        val.clear();
                    }
                    if c == '*' {
                        // only keep track of gears
                        symbols.push(PartNumber {
                            value: -1,
                            loc: Coord::new(x as i32, y as i32),
                            len: 1,
                        });
                    }
                }
            });
            if val_len > 0 {
                parts.push(PartNumber {
                    value: val.parse().unwrap(),
                    loc: Coord::new(line.len() as i32 - 1, y as i32),
                    len: val_len as i32,
                });
                val_len = 0;
                val.clear();
            }
            y += 1;
        });
    let mut sum = 0u32;
    symbols.into_iter().for_each(|symbol| {
        let mut gears = Vec::<i32>::new();
        parts.iter().for_each(|part| {
            if part.is_neighbour(&symbol) {
                gears.push(part.value);
            }
        });
        if gears.len() > 1 {
            // gear ration requires at least 2 gears
            let mut gearing: u32 = gears.pop().unwrap() as u32;
            while let Some(gear) = gears.pop() {
                gearing = gearing * gear as u32;
            }
            sum += gearing;
        }
    });

    return sum;
}

#[cfg(test)]
mod test_d3 {

    #[test]
    pub fn test_d3_p1() {
        let a = super::part1(String::from("data/d3/test_p1.txt"));
        assert_eq!(a, 4361);
    }
    #[test]
    pub fn test_d3_p2() {
        let a = super::part2(String::from("data/d3/test_p2.txt"));
        assert_eq!(a, 467835);
    }

    #[test]
    pub fn test_d3_p1real() {
        let a = super::part1(String::from("data/d3/input.txt"));
        println!("{a}");
        assert_eq!(a, 550934);
    }
}
