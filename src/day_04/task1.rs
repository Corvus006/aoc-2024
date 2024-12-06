use crate::day_04::read_file;

pub fn task1(input: String) -> i32 {
    let data = match read_file(input) {
        Ok(data) => data,
        Err(error) => panic!("There was a problem reading the file: {:?}", error),
    };

    let mut output: i32 = 0;

    for (x, line) in data.iter().enumerate() {
        for (y, &char) in line.iter().enumerate() {
            if char == 'X' {
                output += check(&data, x as i32, y as i32);
            }
        }
    }

    output
}

fn check(data: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let pattern = ['X', 'M', 'A', 'S'];
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // down right
        (1, -1),  // down left
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // up left
        (-1, 1),  // up right
    ];

    let mut found_patterns = 0;

    // checking every direction
    for &(dx, dy) in &directions {
        if matches_pattern(data, x, y, &pattern, dx, dy) {
            found_patterns += 1;
        }
    }

    found_patterns
}

fn matches_pattern(
    data: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    pattern: &[char],
    dx: i32,
    dy: i32,
) -> bool {
    for (i, &char) in pattern.iter().enumerate() {
        let nx = x + i as i32 * dx;
        let ny = y + i as i32 * dy;
        if nx < 0
            || ny < 0
            || nx >= data.len() as i32
            || ny >= data[0].len() as i32
        {
            return false;
        }

        if data[nx as usize][ny as usize] != char {
            return false;
        }
    }

    true
}
