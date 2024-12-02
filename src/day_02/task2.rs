use crate::day_02::read_file;

pub fn task2(input: String) -> i32 {
    // Read the file and handle errors
    let reports: Vec<Vec<i32>> = match read_file(&input) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return 0; // Exit the function if reading fails
        }
    };

    let mut safe_count = 0;

    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
        } else {
            // Check if removing a single level makes the report safe
            for i in 0..report.len() {
                let mut modified_report = report.clone();
                modified_report.remove(i);
                if is_safe(&modified_report) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    safe_count
}

fn is_safe(report: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        // Check if the difference is outside the allowed range
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // Check if levels are increasing or decreasing
        if diff < 0 {
            increasing = false;
        }
        if diff > 0 {
            decreasing = false;
        }
    }

    // A report is safe if it's either strictly increasing or strictly decreasing
    increasing || decreasing
}
