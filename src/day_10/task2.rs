use std::collections::VecDeque;
use crate::day_10::{read_file, trailhead_ratings};
pub fn task2(input:String)-> usize {
    let mut topographic_map = read_file(input).unwrap();

    trailhead_ratings(topographic_map)
}
