use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub mod task1;
pub mod task2;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(PartialEq, Eq)]
struct State {
    score: usize,
    x: i32,
    y: i32,
    di: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn read_input(path: String) -> (Vec<u8>, usize, usize) {
    let input = fs::read_to_string(path).expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines.iter().flat_map(|l| l.as_bytes()).copied().collect::<Vec<_>>();

    (grid, width, height)
}

fn part1(path: String) -> usize {
    let (grid, width, height) = read_input(path);
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'S' {
                start = (x as i32, y as i32);
            } else if grid[y * width + x] == b'E' {
                end = (x as i32, y as i32);
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let start_state = State {
        score: 0,
        x: start.0,
        y: start.1,
        di: 0, // start facing east
    };
    heap.push(Reverse(start_state));

    let mut seen = vec![usize::MAX; width * height * DIRS.len()];
    let mut min = usize::MAX;

    while let Some(Reverse(State {
                               score,
                               x,
                               y,
                               di: prev_di,
                           })) = heap.pop()
    {
        if grid[y as usize * width + x as usize] == b'E' {
            min = score;
            break;
        }

        for (di, (dx, dy)) in DIRS.iter().enumerate() {
            if (prev_di + 2) % DIRS.len() == di {
                continue;
            }

            let nscore = if di == prev_di {
                score + 1
            } else {
                score + 1001
            };

            let nx = x + dx;
            let ny = y + dy;
            let gi = ny as usize * width + nx as usize;
            let si = gi * DIRS.len() + di;
            let last_seen_score = seen[si];

            if grid[gi] != b'#' && nscore <= last_seen_score {
                seen[si] = nscore;
                heap.push(Reverse(State {
                    score: nscore,
                    x: nx,
                    y: ny,
                    di,
                }));
            }
        }
    }

    min
}

type Point = (usize, usize);

fn part2(puzzle_input: &str) -> usize {
    let grid: Vec<Vec<char>> = puzzle_input.lines().map(|line| line.chars().collect()).collect();
    let (m, n) = (grid.len(), grid[0].len());
    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..m {
        for j in 0..n {
            match grid[i][j] {
                'S' => start = (i, j),
                'E' => end = (i, j),
                _ => {}
            }
        }
    }

    let mut grid = grid;
    grid[end.0][end.1] = '.';

    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut heap = BinaryHeap::new();
    let mut visited: HashMap<(usize, Point), usize> = HashMap::new();
    let mut lowest_score = None;
    let mut winning_paths: HashSet<Point> = HashSet::new();

    heap.push(Reverse((0, 0, start.0, start.1, vec![start])));

    while let Some(Reverse((score, d, i, j, path))) = heap.pop() {
        if let Some(lowest) = lowest_score {
            if lowest < score {
                break;
            }
        }

        if (i, j) == end {
            lowest_score = Some(score);
            winning_paths.extend(&path);
            continue;
        }

        let mut can_visit = |d: usize, i: usize, j: usize, new_score: usize| -> bool {
            match visited.get(&(d, (i, j))) {
                Some(&prev_score) if prev_score < new_score => false,
                _ => {
                    visited.insert((d, (i, j)), new_score);
                    true
                }
            }
        };

        let (dx, dy) = directions[d];
        let x = (i as isize + dx) as usize;
        let y = (j as isize + dy) as usize;
        if grid[x][y] == '.' && can_visit(d, x, y, score + 1) {
            let mut new_path = path.clone();
            new_path.push((x, y));
            heap.push(Reverse((score + 1, d, x, y, new_path)));
        }

        let left = (d + 3) % 4;
        if can_visit(left, i, j, score + 1000) {
            heap.push(Reverse((score + 1000, left, i, j, path.clone())));
        }

        let right = (d + 1) % 4;
        if can_visit(right, i, j, score + 1000) {
            heap.push(Reverse((score + 1000, right, i, j, path.clone())));
        }
    }

    winning_paths.len()
}