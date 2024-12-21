use std::fs;
use crate::day_15::solve;

pub fn task2(path: String) -> usize {
    let input: &str = &fs::read_to_string(path).expect("Error reading input");
    let (a, insts) = input.split_once("\n\n").unwrap();
    let g2 = a.lines().map(|l| l.bytes().flat_map(|b| match b {
        b'#' => b"##",
        b'O' => b"[]",
        b'.' => b"..",
        b'@' => b"@.",
        _ => unreachable!(),
    }).copied().collect()).collect();
     solve(g2, insts)
}