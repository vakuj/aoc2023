use std::iter::zip;
use std::path::PathBuf;
use std::{fs, result};

pub fn load_data(file_path: String) -> Result<(Vec<u32>, Vec<u32>), String> {
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from(file_path));
    let binding = fs::read_to_string(path).expect("Should load");
    let lines = binding.lines().collect::<Vec<&str>>();
    // accept only 2 lines
    if lines.len() != 2 {
        return Err(format!("Expected to read 2 lines got {}", lines.len()));
    }
    let mut time = Vec::<u32>::new();
    let mut dist = Vec::<u32>::new();
    if let Some((_, time_str)) = lines[0].split_once(':') {
        time_str.trim().split(' ').for_each(|x| {
            if let Ok(t) = x.trim().parse() {
                time.push(t);
            }
        });
    } else {
        return Err(format!(
            "Could not read time string, see source: {}",
            lines[0]
        ));
    }
    if let Some((_, dist_str)) = lines[1].split_once(':') {
        dist_str.trim().split(' ').for_each(|x| {
            if let Ok(y) = x.trim().parse() {
                dist.push(y);
            }
        });
    } else {
        return Err(format!(
            "Could not read distance string, see source: {}",
            lines[1]
        ));
    }
    Ok((time, dist))
}
fn tot_dist(tot_time: u32, acc_time: u32) -> u32 {
    if tot_time == 0 || acc_time == 0 || acc_time == tot_time {
        return 0;
    }
    let mut vel = 0u32;
    let mut dist = 0u32;
    let mut time = 0u32;
    let mut acc = 0u32;
    while tot_time > time {
        if acc_time > acc {
            acc += 1;
        } else {
            vel = acc;
        }
        dist += vel;
        time += 1;
    }
    return dist;
}

pub fn part1(file_path: String) -> u32 {
    let times: Vec<u32>;
    let dists: Vec<u32>;
    match load_data(file_path) {
        Ok(data) => {
            times = data.0;
            dists = data.1;
        }
        Err(str) => panic!("{str}"),
    }

    let mut result = 0u32;

    for (time, dist) in zip(times, dists) {
        let mut ctr = 0u32;
        for acc_time in 0..time {
            if tot_dist(time, acc_time) > dist {
                ctr += 1;
            }
        }
        if result == 0 {
            result = ctr;
        } else {
            result *= ctr;
        }
    }
    println!("{result}");
    result
}

pub fn part2(file_path: String) -> u32 {
    0
}

#[cfg(test)]
mod test_d6 {
    use crate::d6::load_data;

    #[test]
    pub fn load_test_d6_p1() {
        let time: Vec<u32>;
        let dist: Vec<u32>;
        match load_data(String::from("data/d6/test_p1.txt")) {
            Ok(data) => {
                time = data.0;
                dist = data.1;
            }
            Err(str) => panic!("{str}"),
        }
        assert_eq!(time.len(), dist.len());
        assert_eq!(time.len(), 3);
        assert_eq!(time[0], 7);
        assert_eq!(time[1], 15);
        assert_eq!(time[2], 30);
        assert_eq!(dist[0], 9);
        assert_eq!(dist[1], 40);
        assert_eq!(dist[2], 200);
    }

    #[test]
    pub fn test_d6_p1() {
        let a = super::part1(String::from("data/d6/test_p1.txt"));
        assert_eq!(a, 288);
    }
    #[test]
    pub fn test_d6_p1real() {
        let a = super::part1(String::from("data/d6/input.txt"));
        assert_eq!(a, 227850);
    }
}
