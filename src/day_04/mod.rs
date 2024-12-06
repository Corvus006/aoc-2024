use std::{fs, io::{self, BufRead}};

pub mod task1;
pub mod task2;

pub fn read_file(file_path: String) -> io::Result<Vec<Vec<char>>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut output: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        output.push(chars);
    }

    Ok(output)
}
