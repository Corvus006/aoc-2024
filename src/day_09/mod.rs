pub mod task1;
pub mod task2;

use std::collections::VecDeque;
use std::fs;

fn read_and_preprocess(path: String, filter_digits: bool) -> Vec<char> {
    let mut input = fs::read_to_string(path).expect("Could not read file");
    if filter_digits {
        input = input.chars().filter(|c| c.is_digit(10)).collect();
    }
    input.chars().collect()
}

fn parse_file_blocks(in_chars: &[char]) -> Vec<(usize, usize)> {
    let mut file_blocks = Vec::new();
    let mut idx = 0usize;

    for (i, &ch) in in_chars.iter().enumerate() {
        let ic = ch as i32;
        let size = (ic - 0x30) as usize;
        if size == 0 {
            continue;
        }
        if i % 2 == 0 {
            file_blocks.push((idx, size));
            idx += 1;
        } else {
            file_blocks.push((!0, size));
        }
    }

    file_blocks
}
fn rearrange_file_blocks(file_blocks: &mut Vec<(usize, usize)>) {
    let mut i = file_blocks.len();
    while i > 0 {
        i -= 1;

        let (file_idx, file_size) = file_blocks[i];
        if file_idx == !0 {
            continue;
        }

        for j in 0..i {
            let (free_idx, free_size) = file_blocks[j];
            if free_idx != !0 {
                continue;
            }
            if free_size == file_size {
                file_blocks.swap(i, j);
                break;
            }
            if free_size > file_size {
                file_blocks[j].1 = free_size - file_size;
                file_blocks[i] = (!0, file_size);
                file_blocks.insert(j, (file_idx, file_size));
                i += 1;
                break;
            }
        }
    }
}
fn compute_checksum(file_blocks: &[(usize, usize)]) -> usize {
    let mut checksum = 0usize;
    let mut idx = 0usize;

    for &(file_idx, file_size) in file_blocks {
        if file_idx != !0 {
            for j in 0..file_size {
                checksum += (idx + j) * file_idx;
            }
        }
        idx += file_size;
    }

    checksum
}

fn parse_indices(in_chars: &[char]) -> (Vec<usize>, VecDeque<usize>) {
    let mut file_indices = Vec::new();
    let mut free_indices = VecDeque::new();
    let mut idx = 0;

    for (i, &ch) in in_chars.iter().enumerate() {
        let ic = ch as i32;
        if i % 2 == 0 {
            let used = ic - 0x30;
            for _ in 0..used {
                file_indices.push(idx);
            }
            idx += 1;
        } else {
            let free = ic - 0x30;
            for _ in 0..free {
                free_indices.push_back(file_indices.len());
                file_indices.push(!0);
            }
        }
    }

    (file_indices, free_indices)
}

fn rearrange_indices(file_indices: &mut Vec<usize>, free_indices: &mut VecDeque<usize>) -> usize {
    let mut end = 0usize;

    for i in (0..file_indices.len()).rev() {
        if free_indices.is_empty() {
            end = i + 1;
            break;
        }

        if file_indices[i] == !0 {
            free_indices.pop_back();
        } else {
            let free_idx = free_indices.pop_front().unwrap();
            file_indices.swap(free_idx, i);
        }
    }

    end
}