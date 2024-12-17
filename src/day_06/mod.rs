use std::collections::HashSet;

pub mod task1;
pub mod task2;

fn read_file(file_path: String) -> Option<Vec<Vec<char>>> {
    std::fs::read_to_string(file_path).ok().map(|content| {
        content.lines().map(|line| line.chars().collect()).collect()
    })
}

pub fn get_guard_position(field: &Vec<Vec<char>>) -> (i32, i32) {
    for (y, row) in field.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if "^v<>".contains(ch) {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("Guard not found on the map!");
}

pub fn get_guard_direction(field: &Vec<Vec<char>>, position: &(i32, i32)) -> (i32, i32) {
    match field[position.1 as usize][position.0 as usize] {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Invalid guard direction character!"),
    }
}


fn rotate_right(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        (0, -1) => (1, 0),   // Up -> Right
        (1, 0) => (0, 1),    // Right -> Down
        (0, 1) => (-1, 0),   // Down -> Left
        (-1, 0) => (0, -1),  // Left -> Up
        _ => panic!("Invalid direction"),
    }
}

pub fn guard_step(
    field: &mut Vec<Vec<char>>,
    position: &mut (i32, i32),
    direction: &mut (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
) -> bool {
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
        *direction = rotate_right(*direction); // Rotate right
    } else {
        // Mark current position as visited
        visited.insert(next_position);
        field[position.1 as usize][position.0 as usize] = 'X';

        // Move guard to the next position
        *position = next_position;
    }
    true
}