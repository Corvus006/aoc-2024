use crate::day_05::{load_file, process_input, pages_in_correct_order, middle_page};

pub fn task1(file_path: String) -> usize {
    // Load the file content and handle potential errors.
    let data = load_file(&file_path).expect("Failed to read the file.");

    // Process the input and calculate the result using the provided logic.
    process_input(&data, |pages, rules| {
        if pages_in_correct_order(pages, rules) {
            middle_page(pages)
        } else {
            0
        }
    }) as usize
}
