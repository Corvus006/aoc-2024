use std::fs;
use crate::day_15::solve;

pub fn task1(path: String) -> usize {
    let input: &str = &fs::read_to_string(path).expect("Error reading input");
    let (a, insts) = input.split_once("\n\n").unwrap();
    let g1 = a.lines().map(|l| l.as_bytes().to_vec()).collect();
    solve(g1, insts)
}