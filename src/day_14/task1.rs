use std::fs;
use crate::day_14::{parse, part1};

pub fn task1(input: String) -> usize {
    let mut content = parse(&*fs::read_to_string(input).expect("Could not read file"));
    part1( &content) as usize
}