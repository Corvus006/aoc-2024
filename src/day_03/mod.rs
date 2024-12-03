use std::{fs, io::{self, BufRead}};

pub mod task1;
pub mod task2;
pub fn read_file(file_path: String) -> io::Result<String> {

    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut output = String::new();
    for line in reader.lines() {
        output += &line?;
    }

    Ok(output)
}
