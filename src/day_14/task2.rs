use std::fs;
use crate::day_14::{parse, part2};

pub fn task2(input: String) -> usize {
    let mut content = parse(&*fs::read_to_string(input).expect("Could not read file"));
    part2( &content)
}