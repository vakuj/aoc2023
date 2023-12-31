use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

/// the map type used key pairs to map items
type Map = HashMap<(MapType, MapType), Vec<MapItem>>;
#[derive(Debug)]
/// map item with start of destination and source range and the length of the ranges
struct MapItem {
    /// destination start
    dst: usize,
    /// source start
    src: usize,
    /// length of range
    len: usize,
}
#[derive(Debug, Clone, Copy)]
/// seed range type with start and end
struct ItemRange(usize, usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// map type keys, combine to pairs to use in map
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

/// a record of the keys in the order as they appear
static mut KEY_ORDER: Vec<(MapType, MapType)> = Vec::<(MapType, MapType)>::new();

/// starting seeds for part 1
static mut SEEDS_P1: Vec<usize> = Vec::<usize>::new();

/// starting seed ranges for part 2
static mut SEEDS_P2: Vec<ItemRange> = Vec::<ItemRange>::new();

/// the mapping from keys to map items
static mut MAPPING: Lazy<Map> = Lazy::new(|| {
    let mut map = Map::new();
    let _ = map.insert((MapType::Unknown, MapType::Unknown), Vec::<MapItem>::new());
    map
});
/// flag indicating data is loaded
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

    pub fn get_dst_range(&self, src: &ItemRange) -> Option<ItemRange> {
        match (self.get_dst(src.0), self.get_dst(src.1 - 1)) {
            (None, None) => None,
            (None, Some(end)) => Some(ItemRange {
                0: self.dst,
                1: end,
            }),
            (Some(start), None) => Some(ItemRange {
                0: start,
                1: self.dst + self.len,
            }),
            (Some(start), Some(end)) => Some(ItemRange { 0: start, 1: end }),
        }
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
            SEEDS_P2.push(ItemRange {
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

fn part2_rec(key_offset: usize, next_src: ItemRange) -> Option<ItemRange> {
    if key_offset >= unsafe { KEY_ORDER.len() } {
        return Some(next_src);
    }
    let key = unsafe { KEY_ORDER[key_offset] };
    let mut best_range: Option<ItemRange> = None;
    if unsafe { !MAPPING.contains_key(&key) } {
        println!("==== KEY NOT FOUND: {key:?} ====");
        return None;
    }
    let mut next_srcs = Vec::<ItemRange>::new();
    unsafe { MAPPING.get(&key).unwrap() }
        .into_iter()
        .for_each(|item| match item.get_dst_range(&next_src) {
            Some(ns) => next_srcs.push(ns),
            _ => (),
        });
    if next_srcs.len() == 0 {
        next_srcs.push(next_src);
    }
    next_srcs.into_iter().for_each(|ns| {
        if let Some(curr) = part2_rec(key_offset + 1, ns) {
            if let Some(br) = best_range {
                if curr.0 <= br.0 {
                    best_range = Some(curr);
                }
            } else {
                best_range = Some(curr);
            }
        }
    });
    best_range.or(Some(next_src))
}

pub fn part2(file_path: String) -> usize {
    if unsafe { !DATA_LOADED } {
        load_data(file_path.clone()).expect("Could not load data");
    }
    reload_seeds(file_path).expect("Could not reload seeds");

    let mut best_location: Option<usize> = None;
    for seed in unsafe { &SEEDS_P2 } {
        let range = part2_rec(0, *seed);
        if let Some(best_range) = range {
            if let Some(bl) = best_location {
                if bl > best_range.0 {
                    best_location = Some(best_range.0);
                }
            } else {
                best_location = Some(best_range.0);
            }
        }
    }
    return best_location.unwrap();
}

#[cfg(test)]
mod test_d5 {
    use crate::d5::{
        load_data, reload_seeds, MapType, DATA_LOADED, KEY_ORDER, MAPPING, SEEDS_P1, SEEDS_P2,
    };

    fn reset_loaded() {
        unsafe {
            if DATA_LOADED {
                // clear data if loaded in previous tests
                DATA_LOADED = false;
                KEY_ORDER.clear();
                MAPPING.clear();
                SEEDS_P1.clear();
                SEEDS_P2.clear();
            }
        }
    }

    #[test]
    pub fn test_d5_load() {
        reset_loaded();
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
        reset_loaded();
        let a = super::part1(String::from("data/d5/test_p1.txt"));
        assert_eq!(a, 35);
    }
    #[test]
    pub fn test_d5_p1real() {
        reset_loaded();
        let a = super::part1(String::from("data/d5/input.txt"));
        assert_eq!(a, 323142486);
    }

    #[test]
    pub fn test_d5_p2() {
        reset_loaded();
        let a = super::part2(String::from("data/d5/test_p1.txt"));
        assert_eq!(a, 46);
        //  too high on real input
    }
    #[test]
    pub fn test_d5_p2real() {
        reset_loaded();
        let a = super::part2(String::from("data/d5/input.txt"));
        assert!(26261744 < a && a < 283658805);
        assert_eq!(a, 79874951);
    }
}
