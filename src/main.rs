use std::env;
#[allow(unused)]
use std::fs;
#[allow(unused)]
use std::path::{Path, PathBuf};
use std::{fs::OpenOptions, io::Write};

pub mod d1;
pub mod d2;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Supply a day number to run");
        return;
    }

    let day_arg = &args[1];
    let day: u32 = match day_arg.parse() {
        Ok(d) => d,
        Err(_err) => {
            println!("Could not parse input argument number {day_arg} : {_err}");
            0
        }
    };

    let result_str: String;
    match day {
        1 => {
            let p1 = d1::part1(format!("data/d{day}/input.txt"));
            let p2 = d1::part2(format!("data/d{day}/input.txt"));
            result_str = format!("Day {}\nPart 1: {}\nPart 2: {}\n", day, p1, p2);
        }
        2 => {
            let p1 = d2::part1(format!("data/d{day}/input.txt"));
            let p2 = 0u32;
            // let p2 = d2::part2(format!("data/d{day}/input.txt"));
            result_str = format!("Day {}\nPart 1: {}\nPart 2: {}\n", day, p1, p2);
        }
        _ => return,
    }
    let mut output_path = env::current_dir().unwrap();

    let output_file = format!("output/output{day}.txt");
    output_path.push(PathBuf::from(output_file));

    println!("{result_str}");
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(output_path.as_path())
        .unwrap();
    file.write_all(result_str.as_bytes()).unwrap();
    println!("Task done");
}
