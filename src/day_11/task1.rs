use crate::day_11::{ n_times_blink_optimized, read_file};

pub fn task1(path: String) -> usize {
    let input = read_file(&path);

    let  new_values = input;
     n_times_blink_optimized(new_values,25)

}