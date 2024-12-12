pub mod task1;
pub mod task2;


// Function to parse a single line into target and numbers
pub(crate) fn parse_line(line: &str) -> Option<(usize, Vec<usize>)> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let target = parts[0].trim().parse().ok()?;
    let numbers = parts[1]
        .trim()
        .split_whitespace()
        .map(|num| num.parse().ok())
        .collect::<Option<Vec<usize>>>()?;
    Some((target, numbers))
}

// Function to check if a target can be formed using the numbers
pub(crate) fn can_form_target(numbers: &[usize], target: usize) -> bool {
    // Delegate to the recursive helper function
    evaluate_possible_combinations(numbers, 0, numbers[0], target)
}

// Recursive helper function to evaluate all possible combinations
fn evaluate_possible_combinations(numbers: &[usize], index: usize, current: usize, target: usize) -> bool {
    if index == numbers.len() - 1 {
        return current == target;
    }

    let next = numbers[index + 1];

    // Try addition and multiplication for the next step
    evaluate_possible_combinations(numbers, index + 1, current + next, target)
        || evaluate_possible_combinations(numbers, index + 1, current * next, target)
}
