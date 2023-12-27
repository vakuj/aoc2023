use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

type Map = HashMap<(MapType, MapType), Vec<MapItem>>;

#[derive(Debug)]
struct MapItem {
    dst: usize,
    src: usize,
    len: usize,
}
impl MapItem {
    pub fn from_string(input: &str) -> Self {
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
impl MapType {
    pub fn from_string(mtype: &str) -> MapType {
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
}

pub fn part1(file_path: String) -> usize {
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));

    let content = fs::read_to_string(path).expect("Could not read file");

    let mut nl_count = 0u32;
    let mut incr_flag = false;

    let mut seeds = Vec::<usize>::new();
    let mut key_order = Vec::<(MapType, MapType)>::new();

    let mut mappings = Map::new();
    let mut dst_type = MapType::Unknown;
    let mut src_type = MapType::Unknown;
    let mut key = (dst_type, src_type);

    for line in content.lines() {
        if line.len() <= 1 {
            nl_count += 1;
            incr_flag = true;
            continue;
        }
        if incr_flag {
            incr_flag = false;

            let (maps, _) = line.split_once(' ').unwrap();
            let (src_str, dst_str) = maps.split_once("-to-").unwrap();
            src_type = MapType::from_string(src_str);
            dst_type = MapType::from_string(dst_str);
            assert_ne!(src_type, MapType::Unknown);
            assert_ne!(dst_type, MapType::Unknown);
            key = (src_type, dst_type);
            key_order.push(key);
            continue;
        }

        if nl_count == 0 {
            let (_, nbrs_str) = line.split_once(':').expect("Seeds parsing failed");
            nbrs_str
                .trim()
                .split(' ')
                .into_iter()
                .for_each(|nbr_str| seeds.push(nbr_str.trim().parse().unwrap()));
            continue;
        }
        if !mappings.contains_key(&key) {
            mappings.insert(key, Vec::<MapItem>::new());
        }
        mappings
            .get_mut(&key)
            .unwrap()
            .push(MapItem::from_string(line));
    }
    let mut min_seed: Option<usize> = None;
    let mut next_seed: usize = 0usize;
    for seed in seeds {
        next_seed = seed;
        for key in &key_order {
            if !mappings.contains_key(&key) {
                panic!("key not found");
            }
            for item in mappings.get(key).unwrap() {
                if let Some(ns) = item.get_dst(next_seed) {
                    next_seed = ns;
                    break;
                }
            }
        }

        if let Some(ms) = min_seed {
            if ms > next_seed {
                min_seed = Some(next_seed);
            }
        } else {
            min_seed = Some(next_seed);
        }
    }

    // mappings.into_iter().for_each(|map| println!("{map:?}"));

    return min_seed.unwrap();
}

pub fn part2(file_path: String) -> usize {
    0
}

#[cfg(test)]
mod test_d5 {
    #[test]
    pub fn test_d5_p1() {
        let a = super::part1(String::from("data/d5/test_p1.txt"));
        assert_eq!(a, 35);
    }
}
