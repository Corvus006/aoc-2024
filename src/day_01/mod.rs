use std::{fs, io};
use std::io::BufRead;
use std::path::Path;

pub mod task1;
pub mod task2;

struct Lists {
    list1: Vec<i32>,
    list2: Vec<i32>,
}

fn read_file(file_path: String) -> io::Result<Lists> {
    // Open the file
    println!("1");
    let path_obj = Path::new(&file_path);
    if path_obj.exists() {
        println!("Path exists.");
    } else {
        println!("Path does not exist.");
    }
    let file = fs::File::open(file_path)?;
    println!("2");
    let reader = io::BufReader::new(file);
    println!("3");

    // Initialize the two lists
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    // Process each line
    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() == 2 {
            // Try to parse the numbers and add to the lists
            match (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
                (Ok(num1), Ok(num2)) => {
                    list1.push(num1);
                    list2.push(num2);
                }
                _ => {
                    eprintln!("Skipping invalid line {} (cannot parse numbers): {}", index + 1, line);
                }
            }
        } else {
            eprintln!("Skipping invalid line {} (wrong format): {}", index + 1, line);
        }
    }

    Ok(Lists { list1, list2 })
}
