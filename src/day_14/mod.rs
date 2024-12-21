use std::cmp::Ordering::*;

pub mod task1;
pub mod task2;


type Robot = [usize; 4];

pub fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let position = parts.next()?.strip_prefix("p=")?.split_once(',')?;
            let velocity = parts.next()?.strip_prefix("v=")?.split_once(',')?;

            Some([
                position.0.parse::<usize>().ok()?,
                position.1.parse::<usize>().ok()?,
                velocity.0.parse::<i32>().ok()?.rem_euclid(101) as usize,
                velocity.1.parse::<i32>().ok()?.rem_euclid(103) as usize,
            ])
        })
        .collect()
}



pub fn part1(input: &[Robot]) -> i32 {
    let mut quadrants = [0; 4];

    for [x, y, dx, dy] in input {
        let x = (x + 100 * dx) % 101;
        let y = (y + 100 * dy) % 103;

        match (x.cmp(&50), y.cmp(&51)) {
            (Less, Less) => quadrants[0] += 1,
            (Less, Greater) => quadrants[1] += 1,
            (Greater, Less) => quadrants[2] += 1,
            (Greater, Greater) => quadrants[3] += 1,
            _ => (),
        }
    }

    quadrants.iter().product()
}

pub fn part2(robots: &[Robot]) -> usize {
    let mut rows = Vec::new();
    let mut columns = Vec::new();

    for time in 0..103 {
        let mut xs = [0; 101];
        let mut ys = [0; 103];

        for [x, y, dx, dy] in robots {
            let x = (x + time * dx) % 101;
            xs[x] += 1;
            let y = (y + time * dy) % 103;
            ys[y] += 1;
        }

        // Tree bounding box is 31x33.
        if time < 101 && xs.iter().filter(|&&c| c >= 33).count() >= 2 {
            columns.push(time);
        }
        if ys.iter().filter(|&&c| c >= 31).count() >= 2 {
            rows.push(time);
        }
    }

    if rows.len() == 1 && columns.len() == 1 {
        let t = columns[0];
        let u = rows[0];
        return (5253 * t + 5151 * u) % 10403;
    }

    let mut floor = vec![0; 10403];

    for &t in &columns {
        'outer: for &u in &rows {
            let time = (5253 * t + 5151 * u) % 10403;

            for &[x, y, dx, dy] in robots {
                let x = (x + time * dx) % 101;
                let y = (y + time * dy) % 103;

                let index = 101 * y + x;
                if floor[index] == time {
                    continue 'outer;
                }
                floor[index] = time;
            }

            return time;
        }
    }

    unreachable!()
}