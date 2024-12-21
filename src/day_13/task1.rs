use crate::day_13::{calculate_prize, get_input, solve_combinations, ClawMachine};

pub fn task1(path: String) -> usize {

    let input = get_input(path);
    let solutions=solve_combinations(input);
    calculate_prize(solutions) as usize
}