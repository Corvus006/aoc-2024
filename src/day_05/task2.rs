use std::cmp::Ordering;
use crate::day_05::{load_file, matches_order, pages_in_correct_order, process_input, middle_page};

pub fn task2(file_path: String) -> i32 {
    // Load the file content and handle potential errors.
    let data = load_file(&file_path).expect("Failed to read the file.");

    // Process the input and calculate the result using the provided logic.
    process_input(&data, |pages, rules| {
        if !pages_in_correct_order(pages, rules) {
            sort_pages(pages, rules);
            middle_page(pages)
        } else {
            0
        }
    })
}

fn sort_pages(pages: &mut Vec<i32>, rules: &[(i32, i32)]) {
    // Sort pages based on the custom order defined by the rules.
    pages.sort_by(|&a, &b| {
        if matches_order(rules, (a, b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}
