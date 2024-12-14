use std::collections::{HashSet};
use crate::day_06::{get_guard_direction, get_guard_position, read_file, rotate_right};

pub fn detect_cycle(
    field: &mut Vec<Vec<char>>,
    mut position: (i32, i32),
    mut direction: (i32, i32),
) -> bool {
    let mut visited_obstacles: Vec<(i32, i32, (i32, i32))> = Vec::new();

    loop {
        let next_position = (
            position.0 + direction.0,
            position.1 + direction.1,
        );

        // Check if the next position is out of bounds
        if next_position.1 < 0
            || next_position.1 >= field.len() as i32
            || next_position.0 < 0
            || next_position.0 >= field[0].len() as i32
        {
            return false; // Guard leaves the map
        }

        // Check if the next position is a wall
        if field[next_position.1 as usize][next_position.0 as usize] == '#' {
            direction = rotate_right(direction); // Rotate right

            // Record obstacle and direction
            if visited_obstacles.contains(&(position.0, position.1, direction)) {
                return true; // Loop detected
            }

            visited_obstacles.push((position.0, position.1, direction));
        } else {
            // Move the guard
            position = next_position;
        }
    }
}

pub fn task2(file_path: String) -> usize {
    let mut field = read_file(file_path).expect("Failed to read the file.");
    let guard_start = get_guard_position(&field);
    let guard_start_direction = get_guard_direction(&field, &guard_start);

    // Set to track all fields visited by the guard
    let mut visited_positions = HashSet::new();
    let mut guard_position = guard_start;
    let mut guard_direction = guard_start_direction;

    // Simulate the guard's movement to track its path
    while guard_step(
        &mut field,
        &mut guard_position,
        &mut guard_direction,
        &mut visited_positions,
    ) {}

    // Now, try placing obstacles only on the visited positions
    let mut valid_positions = HashSet::new();
    for &(x, y) in &visited_positions {
        // Skip the guard's starting position
        if (x, y) == guard_start {
            continue;
        }

        // Clone the field and place a temporary obstacle
        let mut field_with_obstacle = field.clone();
        field_with_obstacle[y as usize][x as usize] = '#';

        // Check if the guard gets stuck in a loop
        if detect_cycle(
            &mut field_with_obstacle,
            guard_start,
            guard_start_direction,
        ) {
            valid_positions.insert((x, y));
        }
    }

    valid_positions.len()
}

fn guard_step(
    field: &mut Vec<Vec<char>>,
    position: &mut (i32, i32),
    direction: &mut (i32, i32),
    visited_positions: &mut HashSet<(i32, i32)>,
) -> bool {
    let next_position = (position.0 + direction.0, position.1 + direction.1);

    if next_position.1 < 0
        || next_position.1 >= field.len() as i32
        || next_position.0 < 0
        || next_position.0 >= field[0].len() as i32
    {
        return false; // Guard leaves the map
    }

    if field[next_position.1 as usize][next_position.0 as usize] == '#' {
        *direction = rotate_right(*direction);
        true
    } else {
        *position = next_position;
        visited_positions.insert(*position);
        true
    }
}
