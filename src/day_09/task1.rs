use crate::day_09::{parse_indices, read_and_preprocess, rearrange_indices};

pub fn task1(path: String) -> usize {
    let in_chars = read_and_preprocess(path, false);
    let (mut file_indices, mut free_indices) = parse_indices(&in_chars);
    let end = rearrange_indices(&mut file_indices, &mut free_indices);
    file_indices.drain(end..);

    file_indices
        .iter()
        .enumerate()
        .map(|(i, &oc)| i * oc)
        .sum()
}