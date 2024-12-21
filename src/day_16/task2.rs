use std::fs;
use crate::day_16::{part2};

pub fn task2(path: String) -> usize {
    part2(&*fs::read_to_string(path).expect("Could not read file"))
}