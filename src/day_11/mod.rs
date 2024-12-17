use std::collections::HashMap;

pub mod task1;
pub mod task2;

fn read_file(file: &str) -> Vec<u64> {
    let content = std::fs::read_to_string(file).unwrap();
    content
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok()) // Convert to u64, skip invalid entries
        .collect()
}

fn split_even_digits(n: u64) -> (u64, u64) {
    let digits = n.to_string();
    let mid = digits.len() / 2;
    let left = digits[..mid].parse::<u64>().unwrap_or(0);
    let right = digits[mid..].parse::<u64>().unwrap_or(0);
    (left, right)
}

pub fn n_times_blink_optimized(input: Vec<u64>, iterations: usize) -> usize {
    let mut stones: HashMap<u64, usize> = HashMap::new();

    // Initialize the stone counts
    for stone in input {
        *stones.entry(stone).or_insert(0) += 1;
    }

    // Process stones for the given number of iterations
    for _ in 0..iterations {
        let mut new_stones: HashMap<u64, usize> = HashMap::new();

        for (&stone, &count) in &stones {
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let (left, right) = split_even_digits(stone);
                *new_stones.entry(left).or_insert(0) += count;
                *new_stones.entry(right).or_insert(0) += count;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stones = new_stones; // Swap the new stones
    }

    stones.values().sum()
}