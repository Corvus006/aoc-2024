use std::fs;

pub mod task1;
pub mod task2;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
#[derive(Debug)]
struct ClawMachine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

pub fn get_input(path: String) -> Vec<ClawMachine> {
    let data = fs::read_to_string(path).expect("Failed to read the file");
    let mut machines = Vec::new();

    let lines: Vec<&str> = data.lines().map(str::trim).filter(|line| !line.is_empty()).collect();
    if lines.len() % 3 != 0 {
        panic!("Input file does not contain complete sets of Button A, Button B, and Prize lines");
    }

    for chunk in lines.chunks(3) {
        let button_a_line = chunk[0];
        let button_b_line = chunk[1];
        let prize_line = chunk[2];

        let button_a = parse_point(button_a_line.strip_prefix("Button A:").expect("Invalid Button A format").trim());
        let button_b = parse_point(button_b_line.strip_prefix("Button B:").expect("Invalid Button B format").trim());
        let prize = parse_point(prize_line.strip_prefix("Prize:").expect("Invalid Prize format").trim());

        machines.push(ClawMachine {
            button_a,
            button_b,
            prize,
        });
    }

    machines
}


fn parse_point(input: &str) -> Point {
    let coords: Vec<&str> = input.split(',').collect();
    if coords.len() != 2 {
        panic!("Unexpected point format: {}", input);
    }

    let x: f64= coords[0]
        .trim()
        .strip_prefix("X+")
        .or_else(|| coords[0].trim().strip_prefix("X="))
        .expect("Missing 'X+' or 'X=' prefix")
        .parse()
        .expect("Failed to parse X value");

    let y: f64 = coords[1]
        .trim()
        .strip_prefix("Y+")
        .or_else(|| coords[1].trim().strip_prefix("Y="))
        .expect("Missing 'Y+' or 'Y=' prefix")
        .parse()
        .expect("Failed to parse Y value");

    Point { x, y }
}

fn solve_combination(v1: Point, v2: Point, target: Point) -> Option<(f64, f64)> {

    let a1 = v1.x;

    let b1 = v2.x;
    let c1 = target.x;

    let a2 = v1.y;
    let b2 = v2.y;
    let c2 = target.y;

    let det:f64 = a1 * b2 - a2 * b1;

    if det.abs() < 1e-10 {
        return None;
    }
    // Cramer's Rule
    let a = (c1 * b2 - c2 * b1) / det;
    let b = (a1 * c2 - a2 * c1) / det;

    Some((a, b))
}

fn solve_combinations(claws: Vec<ClawMachine>) -> Vec<(f64, f64)> {
    let mut solution:Vec<(f64, f64)> = Vec::new();
    for claw in claws {
        solution.push(solve_combination(claw.button_a,claw.button_b,claw.prize).unwrap());
    }

    solution
}
fn calculate_prize(solutions: Vec<(f64, f64)>) -> f64 {
    let mut output: Vec<(f64, f64)> = solutions;

    for (a, b) in &mut output {
        if *a % 1.0 != 0.0 || *b % 1.0 != 0.0 {
            *a = 0.0;
            *b = 0.0;
        } else if *a >= 100.0 || *b >= 100.0 {
            *a = 0.0;
            *b = 0.0;
        } else {
            *a *= 3.0;
        }
    }

    output.iter().map(|(a, b)| a + b).sum()
}

fn calculate_prize_2(solutions: Vec<(f64, f64)>) -> f64 {
    let mut output: Vec<(f64, f64)> = solutions;

    for (a, b) in &mut output {
        if *a % 1.0 != 0.0 || *b % 1.0 != 0.0 {
            *a = 0.0;
            *b = 0.0;
        } else {
            *a *= 3.0;
        }
    }

    output.iter().map(|(a, b)| a + b).sum()
}
fn modify_2(claws: Vec<ClawMachine>) -> Vec<ClawMachine> {
    claws
        .into_iter()
        .map(|mut claw| {
            claw.prize.x += 10000000000000.0;
            claw.prize.y += 10000000000000.0;
            claw
        })
        .collect()
}
