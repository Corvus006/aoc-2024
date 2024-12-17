use std::collections::{ HashSet, VecDeque};

pub mod task1;
pub mod task2;

pub struct Map {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
pub struct Region {
    plant_type: char,
    coords: HashSet<(usize, usize)>,
    area: usize,
}

fn voronoi_tesselation(grid: &Vec<Vec<char>>, rows: usize, cols: usize) -> Vec<Region> {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)]; // Handle wrapping for usize subtraction

    for row in 0..rows {
        for col in 0..cols {
            if visited.contains(&(row, col)) {
                continue;
            }

            let plant_type = grid[row][col];
            let mut coords = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((row, col));

            while let Some((r, c)) = queue.pop_front() {
                if visited.contains(&(r, c)) {
                    continue;
                }

                visited.insert((r, c));
                coords.insert((r, c));

                for &(dr, dc) in &directions {
                    let new_r = r.wrapping_add(dr);
                    let new_c = c.wrapping_add(dc);

                    if new_r < rows && new_c < cols &&
                        grid[new_r][new_c] == plant_type &&
                        !visited.contains(&(new_r, new_c)) {
                        queue.push_back((new_r, new_c));
                    }
                }
            }

            regions.push(Region {
                plant_type,
                area: coords.len(),
                coords,
            });
        }
    }

    regions
}

pub fn calculate_total_cost(regions: &[Region], map: &Map) -> usize {
    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    let mut total_cost = 0;

    for region in regions {
        let mut perimeter = 0;

        for &(r, c) in &region.coords {
            for &(dr, dc) in &directions {
                let new_r = r.wrapping_add(dr);
                let new_c = c.wrapping_add(dc);

                if new_r >= map.rows || new_c >= map.cols || map.grid[new_r][new_c] != region.plant_type {
                    perimeter += 1;
                }
            }
        }

        total_cost += region.area * perimeter;
    }

    total_cost
}

pub fn calculate_total_cost2(regions: &[Region], map: &Map) -> usize {
    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    let mut total_cost = 0;

    for region in regions {
        let mut edges = HashSet::new();
        for &(r, c) in &region.coords {
            for (idx, (dr, dc)) in directions.iter().enumerate() {
                let new_r = r as i32 + *dr as i32;
                let new_c = c as i32 + *dc as i32;

                if new_r < 0 || new_r >= map.rows as i32 || new_c < 0 || new_c >= map.cols as i32 ||
                    map.grid[new_r as usize][new_c as usize] != region.plant_type {
                    edges.insert((r, c, idx));
                }
            }
        }

        let mut sides = 0;
        let mut visited_edges = HashSet::new();

        for &(r, c, dir) in &edges {
            if visited_edges.contains(&(r, c, dir)) {
                continue;
            }

            sides += 1;

            let perp_dirs = match dir {
                0 | 2 => [(1, 0), (-1, 0)],
                1 | 3 => [(0, 1), (0, -1)],
                _ => unreachable!(),
            };

            for &(dr, dc) in &perp_dirs {
                let mut curr_r = r as i32;
                let mut curr_c = c as i32;

                loop {
                    curr_r += dr;
                    curr_c += dc;

                    if curr_r < 0 || curr_r >= map.rows as i32 ||
                        curr_c < 0 || curr_c >= map.cols as i32 {
                        break;
                    }

                    let curr_coord = (curr_r as usize, curr_c as usize, dir);
                    if !edges.contains(&curr_coord) {
                        break;
                    }

                    visited_edges.insert(curr_coord);
                }
            }
        }

        total_cost += region.area * sides;
    }

    total_cost
}


fn read_file(file_path: String) -> Option<Vec<Vec<char>>> {
    std::fs::read_to_string(file_path).ok().map(|content| {
        content.lines().map(|line| line.chars().collect()).collect()
    })
}



