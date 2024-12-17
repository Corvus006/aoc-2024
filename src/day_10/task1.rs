use crate::day_10::{calculate_trailhead_scores, read_file, };

pub fn task1(input: String) -> usize {
    let  map = read_file(input).expect("Failed to read input");
    calculate_trailhead_scores(map)
}