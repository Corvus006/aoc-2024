use std::collections::HashMap;
use std::fs;
use crate::day_08::{ populate_node_map, print_map, process_nodes_1, read_file, Node};

pub fn task1(input: String) -> usize {
    let mut map = read_file(input);
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    let mut node_map: HashMap<char, Vec<Node>> = HashMap::new();

    populate_node_map(&mut node_map, &map, width, height);

    let mut count = 0;

    for (_, nodes) in node_map {
        count += process_nodes_1(&nodes, &mut map, width, height);
    }

    print_map(&map);

    count
}
