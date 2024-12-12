use std::io::{stdin, stdout, Write};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

fn day_fn(day: i32, task: i32) -> fn(String) -> usize {
    match day {
        1 => match task {
            1 => day_01::task1::task1,
            2 => day_01::task2::task2,
            _ => panic!("Invalid task for day 1"),
        },
        2 => match task {
            1 => day_02::task1::task1,
            2 => day_02::task2::task2,
            _ => panic!("Invalid task for day 2"),
        },
        3 => match task {
            1 => day_03::task1::task1,
            2 => day_03::task2::task2,
            _ => panic!("Invalid task for day 3"),
        },
        4 => match task {
            1 => day_04::task1::task1,
            2 => day_04::task2::task2,
            _ => panic!("Invalid task for day 4"),
        },
        5 => match task {
            1 => day_05::task1::task1,
            2 => day_05::task2::task2,
            _ => panic!("Invalid task for day 5"),
        },
        6 => match task {
            1 => day_06::task1::task1,
            2 => day_06::task2::task2,
            _ => panic!("Invalid task for day 6"),
        },
        7 => match task {
            1 => day_07::task1::task1,
            2 => day_07::task2::task2,
            _ => panic!("Invalid task for day 7"),
        },
        8 => match task {
            1 => day_08::task1::task1,
            2 => day_08::task2::task2,
            _ => panic!("Invalid task for day 8"),
        },
        9 => match task {
            1 => day_09::task1::task1,
            2 => day_09::task2::task2,
            _ => panic!("Invalid task for day 9"),
        },
        10 => match task {
            1 => day_10::task1::task1,
            2 => day_10::task2::task2,
            _ => panic!("Invalid task for day 10"),
        },
        11 => match task {
            1 => day_11::task1::task1,
            2 => day_11::task2::task2,
            _ => panic!("Invalid task for day 11"),
        },
        12 => match task {
            1 => day_12::task1::task1,
            2 => day_12::task2::task2,
            _ => panic!("Invalid task for day 12"),
        },
        13 => match task {
            1 => day_13::task1::task1,
            2 => day_13::task2::task2,
            _ => panic!("Invalid task for day 13"),
        },
        14 => match task {
            1 => day_14::task1::task1,
            2 => day_14::task2::task2,
            _ => panic!("Invalid task for day 14"),
        },
        15 => match task {
            1 => day_15::task1::task1,
            2 => day_15::task2::task2,
            _ => panic!("Invalid task for day 15"),
        },
        16 => match task {
            1 => day_16::task1::task1,
            2 => day_16::task2::task2,
            _ => panic!("Invalid task for day 16"),
        },
        17 => match task {
            1 => day_17::task1::task1,
            2 => day_17::task2::task2,
            _ => panic!("Invalid task for day 17"),
        },
        18 => match task {
            1 => day_18::task1::task1,
            2 => day_18::task2::task2,
            _ => panic!("Invalid task for day 18"),
        },
        19 => match task {
            1 => day_19::task1::task1,
            2 => day_19::task2::task2,
            _ => panic!("Invalid task for day 19"),
        },
        20 => match task {
            1 => day_20::task1::task1,
            2 => day_20::task2::task2,
            _ => panic!("Invalid task for day 20"),
        },
        21 => match task {
            1 => day_21::task1::task1,
            2 => day_21::task2::task2,
            _ => panic!("Invalid task for day 21"),
        },
        22 => match task {
            1 => day_22::task1::task1,
            2 => day_22::task2::task2,
            _ => panic!("Invalid task for day 22"),
        },
        23 => match task {
            1 => day_23::task1::task1,
            2 => day_23::task2::task2,
            _ => panic!("Invalid task for day 23"),
        },
        24 => match task {
            1 => day_24::task1::task1,
            2 => day_24::task2::task2,
            _ => panic!("Invalid task for day 24"),
        },
        25 => match task {
            1 => day_25::task1::task1,
            2 => day_25::task2::task2,
            _ => panic!("Invalid task for day 25"),
        },
        _ => panic!("Invalid day"),
    }
}

fn main() {
    let mut day = String::new();
    let mut task = String::new();

    print!("day: (1..25) ");
    stdout().flush().expect("stdout flush failed");
    stdin().read_line(&mut day).expect("failed to read day");
    {
        let len = day.trim_end_matches(&['\r', '\n'][..]).len();
        day.truncate(len);
    }
    let day_num = day.parse::<i32>().expect("failed to parse day");

    print!("task: (1..2) ");
    stdout().flush().expect("stdout flush failed");
    stdin().read_line(&mut task).expect("failed to read task");
    {
        let len = task.trim_end_matches(&['\r', '\n'][..]).len();
        task.truncate(len);
    }
    let task_num = task.parse::<i32>().expect("failed to parse task");

    // Generate the file path
    let path = format!("data/{:02}/{:02}/data.txt", day_num, task_num);

    // Call the corresponding function and print the result
    let result = day_fn(day_num, task_num)(path);
    println!("result: {result}");
}
