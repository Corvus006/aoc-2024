use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub mod task1;
pub mod task2;

fn read_file(file_path: &str) -> Result<Vec<Vec<i32>>> {
    // Open the file and handle potential errors
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut outer_vector: Vec<Vec<i32>> = Vec::new();

    // Read lines and handle errors
    for line_result in reader.lines() {
        let line = line_result?; // Unwrap the Result<String>
        let inner_vector: Vec<i32> = line
            .split_whitespace() // Split the line by whitespace
            .filter_map(|s| s.parse::<i32>().ok()) // Parse numbers
            .collect();

        outer_vector.push(inner_vector); // Add the inner vector to the outer vector
    }

    Ok(outer_vector) // Return the result
}
