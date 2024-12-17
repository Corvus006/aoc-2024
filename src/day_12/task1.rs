use crate::day_12::{calculate_total_cost, read_file, voronoi_tesselation, Map};

pub fn task1(path: String) -> usize {
    let input = read_file(path).expect("Error reading file");
    let rows = input.len();
    let cols = input[0].len();

    let map = Map {
        grid: input.clone(),
        rows,
        cols,
    };

    let regions = voronoi_tesselation(&input, rows, cols);
    calculate_total_cost(&regions, &map)
}