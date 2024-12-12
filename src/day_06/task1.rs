use std::collections::HashSet;
use crate::day_06::{read_file, get_guard_position, get_guard_direction, guard_step};

pub fn task1(file_path: String) -> usize {
    let mut field = read_file(file_path).expect("Failed to read the file.");
    let mut guard_position = get_guard_position(&field);
    let mut guard_direction = get_guard_direction(&field, &guard_position);
    let mut visited_positions = HashSet::new();

    // Add starting position to visited
    visited_positions.insert(guard_position);

    // Simulate guard movement
    while guard_step(
        &mut field,
        &mut guard_position,
        &mut guard_direction,
        &mut visited_positions,
    ) {}

    visited_positions.len()
}