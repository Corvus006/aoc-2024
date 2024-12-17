pub mod task1;
pub mod task2;

use std::collections::{HashSet, VecDeque};

fn read_file(file_path: String) -> Option<Vec<Vec<u8>>> {
    match std::fs::read_to_string(file_path) {
        Ok(content) => Some(parse_map(&content)),
        Err(_) => None,
    }
}

fn parse_map(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn is_in_bounds(map: &Vec<Vec<u8>>, row: isize, col: isize) -> bool {
    row >= 0 && col >= 0 && (row as usize) < map.len() && (col as usize) < map[0].len()
}

fn bfs_find_reachable_nines(
    map: &Vec<Vec<u8>>,
    start_row: usize,
    start_col: usize,
) -> HashSet<(usize, usize)> {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_row, start_col));
    visited.insert((start_row, start_col));

    let mut reachable_nines = HashSet::new();

    while let Some((row, col)) = queue.pop_front() {
        for (dx, dy) in directions.iter() {
            let new_row = row as isize + dx;
            let new_col = col as isize + dy;

            if !is_in_bounds(map, new_row, new_col) {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;
            if visited.contains(&(new_row, new_col)) {
                continue;
            }

            let current_height = map[row][col];
            let next_height = map[new_row][new_col];

            if next_height == current_height + 1 {
                visited.insert((new_row, new_col));
                queue.push_back((new_row, new_col));
                if next_height == 9 {
                    reachable_nines.insert((new_row, new_col));
                }
            }
        }
    }

    reachable_nines
}

fn calculate_trailhead_scores(map: Vec<Vec<u8>>) -> usize {
    let mut total_score = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 0 {
                let reachable_nines = bfs_find_reachable_nines(&map, row, col);
                total_score += reachable_nines.len();
            }
        }
    }

    total_score
}

fn trailhead_ratings(topographic_map: Vec<Vec<u8>>) -> usize {
    let mut score_sum = 0usize;
    for z in 0..topographic_map.len() {
        for x in 0..topographic_map[0].len() {
            if topographic_map[z][x] != 0 {
                continue;
            }

            let mut next = VecDeque::new();
            next.push_back((x as i32, 0u8, z as i32));

            while !next.is_empty() {
                let (nx, ny, nz) = next.pop_front().unwrap();
                if ny == 9 {
                    score_sum += 1;
                    continue;
                }
                if nx - 1 >= 0 && topographic_map[nz as usize][(nx - 1) as usize] == ny + 1 {
                    next.push_back((nx - 1, ny + 1, nz));
                }
                if nx + 1 < topographic_map[0].len() as i32
                    && topographic_map[nz as usize][(nx + 1) as usize] == ny + 1
                {
                    next.push_back((nx + 1, ny + 1, nz));
                }
                if nz - 1 >= 0 && topographic_map[(nz - 1) as usize][nx as usize] == ny + 1 {
                    next.push_back((nx, ny + 1, nz - 1));
                }
                if nz + 1 < topographic_map.len() as i32
                    && topographic_map[(nz + 1) as usize][nx as usize] == ny + 1
                {
                    next.push_back((nx, ny + 1, nz + 1));
                }
            }
        }
    }

    score_sum
}
