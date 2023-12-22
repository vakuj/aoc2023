#[allow(unused)]
use std::fs;
#[allow(unused)]
use std::path::{Path, PathBuf};

pub mod d1;

fn main() {
    let d1p1 = d1::part1("data/input_d1.txt");
    println!("Part 1: {d1p1}");
}
