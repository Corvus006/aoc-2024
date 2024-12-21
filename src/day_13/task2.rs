use crate::day_13::{ calculate_prize_2, get_input, modify_2, solve_combinations, ClawMachine};

pub fn task2(path: String) -> usize {
    let mut input = get_input(path);
    input = modify_2(input);
    let solutions=solve_combinations(input);
    calculate_prize_2(solutions) as usize
}