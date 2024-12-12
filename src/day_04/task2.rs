use crate::day_04::read_file;

pub fn task2(input: String) -> usize {
    let data = match read_file(input) {
        Ok(data) => data,
        Err(error) => panic!("There was a problem reading the file: {:?}", error),
    };

    let mut output: i32 = 0;

    for (x, line) in data.iter().enumerate() {
        for (y, &char) in line.iter().enumerate() {
            if char == 'A' {
                output += check(&data, x as i32, y as i32);
            }
        }
    }

    output as usize
}
fn check(data: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    // Define the four diagonal patterns
    let patterns = vec![
        // Top-left to bottom-right
        vec![('M', -1, -1), ('S', 1, 1)],
        // Bottom-right to Top-left
        vec![('M', 1, 1), ('S', -1, -1)],
        // Top-right to bottom-left
        vec![('M', -1, 1), ('S', 1, -1)],
        // Bottom-left to Top right
        vec![('M', 1, -1), ('S', -1, 1)],

    ];

    let mut found_patterns = 0;

    for pattern in &patterns {
        if matches_pattern(data, x, y, pattern) {
            found_patterns += 1;
        }
    }

    // To form an "X", both diagonal patterns must exist
    if found_patterns == 2 {
        return 1; // One "X" found
    }

    0 // No "X" found
}

fn matches_pattern(
    data: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    pattern: &Vec<(char, i32, i32)>,
) -> bool {
    for &(expected_char, dx, dy) in pattern {
        let nx = x + dx;
        let ny = y + dy;

        // Boundary check
        if nx < 0 || ny < 0 || nx >= data.len() as i32 || ny >= data[0].len() as i32 {
            return false;
        }

        // Check if the character matches
        if data[nx as usize][ny as usize] != expected_char {
            return false;
        }
    }

    true
}