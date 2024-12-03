use crate::day_03::read_file;

pub fn task1(input: String) -> i32 {
    let code = match read_file(input) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return 0;
        }
    };

    let mut results: Vec<i32> = Vec::new();
    let mut solution: i32 = 0;

    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for caps in re.captures_iter(&code) {
        if let (Some(x), Some(y)) = (caps.get(1), caps.get(2)) {
            if let (Ok(x_val), Ok(y_val)) = (x.as_str().parse::<i32>(), y.as_str().parse::<i32>()) {
                results.push(x_val * y_val);
            }
        }
    }
    println!("{:?}", results);

    solution = results.iter().sum();
    solution
}