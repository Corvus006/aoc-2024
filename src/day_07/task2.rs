use std::fs;
use crate::day_07::parse_line;

pub fn task2(input: String) -> usize {
    let content = fs::read_to_string(input).expect("Could not read file");
    let mut total_calibration_result = 0;

    for line in content.lines() {
        if let Some((target, numbers)) = parse_line(line) {
            if can_form_target_task2(&numbers, target) {
                total_calibration_result += target;
            }
        }
    }

    total_calibration_result
}

fn can_form_target_task2(numbers: &[usize], target: usize) -> bool {
    evaluate_possible_combinations_task2(numbers, 0, numbers[0], target)
}

fn evaluate_possible_combinations_task2(numbers: &[usize], index: usize, current: usize, target: usize) -> bool {
    if index == numbers.len() - 1 {
        return current == target;
    }

    let next = numbers[index + 1];

    evaluate_possible_combinations_task2(numbers, index + 1, current + next, target)
        || evaluate_possible_combinations_task2(numbers, index + 1, current * next, target)
        || evaluate_possible_combinations_task2(numbers, index + 1, concatenate(current, next), target)
}

fn concatenate(left: usize, right: usize) -> usize {
    let right_digits = (10_usize.pow((right as f64).log10().floor() as u32 + 1)) as usize;
    left * right_digits + right
}