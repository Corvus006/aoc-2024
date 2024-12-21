use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

pub mod task1;
pub mod task2;

fn solve(mut g: Vec<Vec<u8>>, insts: &str) -> usize {
    let (mut r, mut c) = (0..g.len()).cartesian_product(0..g[0].len())
        .find(|&(r, c)| g[r][c] == b'@')
        .unwrap();
    'outer: for i in insts.bytes() {
        let (dr, dc) = match i {
            b'^' => (-1,  0),
            b'>' => ( 0,  1),
            b'v' => ( 1,  0),
            b'<' => ( 0, -1),
            _ => continue,
        };
        let mut q = VecDeque::from([(r, c)]);
        let mut seen = HashSet::new();
        while let Some((rr, cc)) = q.pop_front() {
            if !seen.insert((rr, cc)) {
                continue;
            }
            let (r2, c2) = match (rr as isize + dr, cc as isize + dc) {
                (r2, c2) if r2 >= 0 && c2 >= 0 && r2 < g.len() as isize && c2 < g[0].len() as isize => {
                    (r2 as usize, c2 as usize)
                }
                _ => continue,
            };
            match g[r2][c2] {
                b'#' => continue 'outer,
                b'O' => q.push_back((r2, c2)),
                b'[' => q.extend([(r2, c2), (r2, c2 + 1)]),
                b']' => q.extend([(r2, c2), (r2, c2 - 1)]),
                _ => continue,
            }
        }
        let boxes = seen.iter()
            .sorted_by_key(|&&(rr, cc)| (c.abs_diff(cc), r.abs_diff(rr)))
            .rev();
        for &(rr, cc) in boxes {
            let (r2, c2) = match (rr as isize + dr, cc as isize + dc) {
                (r2, c2) if r2 >= 0 && c2 >= 0 && r2 < g.len() as isize && c2 < g[0].len() as isize => {
                    (r2 as usize, c2 as usize)
                }
                _ => continue,
            };

            g[r2][c2] = g[rr][cc];
            g[rr][cc] = b'.';
        }
        if let (Some(new_r), Some(new_c)) = (
            (r as isize + dr).try_into().ok(),
            (c as isize + dc).try_into().ok(),
        ) {
            if new_r < g.len() && new_c < g[0].len() {
                r = new_r;
                c = new_c;
            }
        }

    }
    (0..g.len()).cartesian_product(0..g[0].len())
        .filter(|&(r, c)| matches!(g[r][c], b'O' | b'['))
        .map(|(r, c)| r * 100 + c)
        .sum()
}