use crate::day_02::read_file;

pub fn task1(input: String) -> i32 {
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
        let mut increasing = true;
        let mut decreasing = true;
        let mut valid_differences = true;

        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];

            // Check if the difference is within the allowed range
            if diff.abs() < 1 || diff.abs() > 3 {
                valid_differences = false;
                break;
            }

            // Check if levels are increasing or decreasing
            if diff < 0 {
                increasing = false;
            }
            if diff > 0 {
                decreasing = false;
            }
        }

        // A report is safe if it's strictly increasing or decreasing and has valid differences
        if valid_differences && (increasing || decreasing) {
            safe_count += 1;
        } else {
            println!("Unsafe report: {:?}", report);
        }
    }

    safe_count
}
