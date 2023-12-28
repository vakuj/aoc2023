use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

type Map = HashMap<(MapType, MapType), Vec<MapItem>>;
#[derive(Debug)]
struct MapItem {
    dst: usize,
    src: usize,
    len: usize,
}
#[derive(Debug)]
struct SeedRange(usize, usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum MapType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
    Unknown,
}
static mut KEY_ORDER: Vec<(MapType, MapType)> = Vec::<(MapType, MapType)>::new();
static mut SEEDS_P1: Vec<usize> = Vec::<usize>::new();
static mut SEEDS_P2: Vec<SeedRange> = Vec::<SeedRange>::new();
static mut MAPPING: Lazy<Map> = Lazy::new(|| {
    let mut map = Map::new();
    let _ = map.insert((MapType::Unknown, MapType::Unknown), Vec::<MapItem>::new());
    map
});
static mut DATA_LOADED: bool = false;

impl MapItem {
    pub fn parse(input: &str) -> Self {
        let tmp: Vec<&str> = input.split(' ').collect();
        assert_eq!(tmp.len(), 3);
        MapItem {
            dst: tmp[0].parse().unwrap(),
            src: tmp[1].parse().unwrap(),
            len: tmp[2].parse().unwrap(),
        }
    }

    pub fn get_dst(&self, src: usize) -> Option<usize> {
        if self.src <= src && src < self.src + self.len {
            return Some(self.dst + (src - self.src));
        }
        None
    }
}
impl MapType {
    pub fn parse(mtype: &str) -> MapType {
        match mtype {
            "seed" => MapType::Seed,
            "soil" => MapType::Soil,
            "fertilizer" => MapType::Fertilizer,
            "water" => MapType::Water,
            "light" => MapType::Light,
            "temperature" => MapType::Temperature,
            "humidity" => MapType::Humidity,
            "location" => MapType::Location,
            _ => MapType::Unknown,
        }
    }

    pub fn parse_pair(pair: &str) -> (MapType, MapType) {
        let (src_str, dst_str) = pair.split_once("-to-").unwrap();
        (MapType::parse(src_str), MapType::parse(dst_str))
    }
}

pub fn load_data(file_path: String) -> Result<(), Error> {
    if unsafe { DATA_LOADED } {
        return Ok(());
    }
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));

    let result = fs::read_to_string(path);
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    let content = result.unwrap();

    // let mut nl_count = 0u32;
    let mut flag = false;

    let mut key = (MapType::Unknown, MapType::Unknown);

    let mut lines = content.lines();
    let line = lines.next();
    // read the first line and populate seeds
    let (_, nbrs_str) = line.unwrap().split_once(':').expect("Seeds parsing failed");
    nbrs_str
        .trim()
        .split(' ')
        .into_iter()
        .for_each(|nbr_str| unsafe { SEEDS_P1.push(nbr_str.trim().parse().unwrap()) });

    while let Some(line) = lines.next() {
        if line.len() <= 1 {
            // new line found, mark flag to read mapping
            flag = true;
            continue;
        }
        if flag {
            // read mapping, unmark flag to continue read map-items
            flag = false;
            key = MapType::parse_pair(line.split_once(' ').unwrap().0);
            unsafe { KEY_ORDER.push(key) };
            continue;
        }

        unsafe {
            // add new key if not found
            if !MAPPING.contains_key(&key) {
                MAPPING.insert(key, Vec::<MapItem>::new());
            }
            // add new map-item to the current key
            MAPPING.get_mut(&key).unwrap().push(MapItem::parse(line));
        }
    }
    unsafe { DATA_LOADED = true };
    Ok(())
}

pub fn reload_seeds(file_path: String) -> Result<(), Error> {
    if unsafe { !DATA_LOADED } {
        load_data(file_path).expect("Could not load data");
    }

    unsafe { DATA_LOADED = false };

    for ii in (0..unsafe { SEEDS_P1.len() }).step_by(2) {
        unsafe {
            SEEDS_P2.push(SeedRange {
                0: SEEDS_P1[ii],
                1: SEEDS_P1[ii] + SEEDS_P1[ii + 1],
            });
        }
    }
    unsafe { DATA_LOADED = true };
    Ok(())
}

pub fn part1(file_path: String) -> usize {
    if unsafe { !DATA_LOADED } {
        load_data(file_path).expect("Could not load data");
    }
    let mut best_location: Option<usize> = None;
    let mut next_src: usize;
    for seed in unsafe { &SEEDS_P1 } {
        next_src = *seed;
        for key in unsafe { &KEY_ORDER } {
            // panic if key not found, should be there...
            if unsafe { !MAPPING.contains_key(&key) } {
                // panic!("key not found");
                print!("{:?} not found,", key);
                continue;
            }
            // try to find next destination by evaluating the sources of
            // the items related to this key. if not found the source and
            // destination are mapped to the same value.
            for item in unsafe { MAPPING.get(&key).unwrap() } {
                if let Some(ns) = item.get_dst(next_src) {
                    next_src = ns;
                    break;
                }
            }
        }

        if let Some(bl) = best_location {
            if bl > next_src {
                best_location = Some(next_src);
            }
        } else {
            best_location = Some(next_src);
        }
    }
    return best_location.unwrap();
}

pub fn part2(file_path: String) -> usize {
    if unsafe { !DATA_LOADED } {
        load_data(file_path.clone()).expect("Could not load data");
    }
    reload_seeds(file_path).expect("Could not reload seeds");

    let mut best_location: Option<usize> = None;
    let mut next_src: usize;
    for sr in unsafe { &SEEDS_P2 } {
        for seed in sr.0..sr.1 {
            next_src = seed;
            for key in unsafe { &KEY_ORDER } {
                // panic if key not found, should be there...
                if unsafe { !MAPPING.contains_key(&key) } {
                    panic!("key not found");
                }
                // try to find next destination by evaluating the sources of
                // the items related to this key. if not found the source and
                // destination are mapped to the same value.
                for item in unsafe { MAPPING.get(&key).unwrap() } {
                    if let Some(ns) = item.get_dst(next_src) {
                        println!("{key:?} : {next_src:?} -> {ns:?}");
                        next_src = ns;
                        break;
                    }
                }
            }

            if let Some(bl) = best_location {
                if bl > next_src {
                    best_location = Some(next_src);
                }
            } else {
                best_location = Some(next_src);
            }
            println!("---- {best_location:?} [{sr:?})");
        }
    }
    return best_location.unwrap();
}

#[cfg(test)]
mod test_d5 {
    use crate::d5::{load_data, reload_seeds, MapType, KEY_ORDER, SEEDS_P1, SEEDS_P2};

    #[test]
    pub fn test_d5_load() {
        let result = load_data(String::from("data/d5/test_p1.txt"));
        assert!(result.is_ok());

        assert_eq!(unsafe { SEEDS_P1.len() }, 4);
        assert_eq!(unsafe { SEEDS_P1[0] }, 79);
        assert_eq!(unsafe { SEEDS_P1[1] }, 14);
        assert_eq!(unsafe { SEEDS_P1[2] }, 55);
        assert_eq!(unsafe { SEEDS_P1[3] }, 13);

        assert_eq!(unsafe { KEY_ORDER.len() }, 7);
        assert_eq!(unsafe { KEY_ORDER[0] }, (MapType::Seed, MapType::Soil));
        assert_eq!(
            unsafe { KEY_ORDER[6] },
            (MapType::Humidity, MapType::Location)
        );

        let result = reload_seeds(String::from("data/d5/test_p1.txt"));
        assert!(result.is_ok());
        assert_eq!(unsafe { SEEDS_P2.len() }, 2);
    }

    #[test]
    pub fn test_d5_p1() {
        let a = super::part1(String::from("data/d5/test_p1.txt"));
        assert_eq!(a, 35);
    }
    #[test]
    pub fn test_d5_p2() {
        let a = super::part2(String::from("data/d5/test_p1.txt"));
        assert_eq!(a, 46);
    }
}
