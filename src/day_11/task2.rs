use crate::day_11::{ n_times_blink_optimized, read_file};

pub fn task2(path: String) -> usize {
    let input = read_file(&path);

    n_times_blink_optimized(input, 75)
}