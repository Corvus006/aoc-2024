use crate::day_09::{compute_checksum, parse_file_blocks, read_and_preprocess, rearrange_file_blocks};

pub fn task2(path: String) -> usize {
    let in_chars = read_and_preprocess(path,true);
    let mut file_blocks = parse_file_blocks(&in_chars);

    rearrange_file_blocks(&mut file_blocks);

    compute_checksum(&file_blocks)
}