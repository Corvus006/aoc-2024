use std::fs;
use crate::day_07::{can_form_target, parse_line};

pub fn task1(input: String) -> usize {
    // Read the content of the file
    let content = fs::read_to_string(input).expect("Could not read file");

    let mut total_calibration_result = 0;

    for line in content.lines() {
        // Parse each line to extract the target and numbers
        if let Some((target, numbers)) = parse_line(line) {
            // Check if the target can be formed using the numbers
            if can_form_target(&numbers, target) {
                total_calibration_result += target;
            }
        }
    }

    total_calibration_result
}
