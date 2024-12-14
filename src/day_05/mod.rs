use std::{fs, io::{self}};
use std::io::Read;

pub mod task1;
pub mod task2;

fn load_file(file_path: &str) -> io::Result<String> {
    let mut file_content = String::new();
    fs::File::open(file_path)?.read_to_string(&mut file_content)?;
    Ok(file_content)
}

fn matches_order(rules: &[(i32, i32)], pair: (i32, i32)) -> bool {
    rules
        .iter()
        .any(|&(a, b)| (a == pair.0 && b == pair.1) || (b == pair.0 && a == pair.1))
        .then_some(rules.iter().any(|&(a, b)| a == pair.0 && b == pair.1))
        .unwrap_or(true)
}

fn pages_in_correct_order(pages: &[i32], rules: &[(i32, i32)]) -> bool {
    pages.is_sorted_by(|&x, &y| matches_order(rules, (x, y)))
}

fn middle_page(pages: &[i32]) -> i32 {
    pages[pages.len() / 2]
}

fn process_input(input: &str, page_processor: fn(&mut Vec<i32>, &[(i32, i32)]) -> i32) -> i32 {
    let (rules, results) = input.lines().fold((Vec::new(), Vec::new()), |(mut rules, mut results), line| {
        if line.len() == 5 {
            let (a, b): (i32, i32) = (
                line[..2].parse().unwrap(),
                line[3..].parse().unwrap(),
            );
            rules.push((a, b));
        } else {
            let mut pages = line
                .split(',')
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<_>>();

            if !pages.is_empty() {
                results.push(page_processor(&mut pages, &rules));
            }
        }
        (rules, results)
    });

    results.into_iter().sum()
}
