use std::collections::HashMap;
use std::fs;

pub mod task1;
pub mod task2;


#[derive(PartialEq)]
struct Node {
    x: i32,
    y: i32,
}

fn antenna_vec(a: &Node, b: &Node) -> (i32, i32) {
    (b.x - a.x, b.y - a.y)
}

fn extend_1(a: &Node, b: &Node) -> (Node, Node) {
    let vec = antenna_vec(a, b);
    let a_a = Node {
        x: a.x - vec.0,
        y: a.y - vec.1,
    };
    let a_b = Node {
        x: b.x + vec.0,
        y: b.y + vec.1,
    };
    (a_a, a_b)
}

fn extend_2(a: &Node, b: &Node, w: i32, h: i32) -> Vec<Node> {
    let vec = antenna_vec(a, b);

    let mut result = Vec::new();

    let mut pos = (a.x, a.y);
    while inside_2(&pos, w, h) {
        result.push(Node { x: pos.0, y: pos.1 });
        pos.0 -= vec.0;
        pos.1 -= vec.1;
    }

    pos = (b.x, b.y);
    while inside_2(&pos, w, h) {
        result.push(Node { x: pos.0, y: pos.1 });
        pos.0 += vec.0;
        pos.1 += vec.1;
    }

    result
}
fn inside_2(pos: &(i32, i32), w: i32, h: i32) -> bool {
    pos.0 >= 0 && pos.0 < w && pos.1 >= 0 && pos.1 < h
}
fn inside_1(node: &Node, w: i32, h: i32) -> bool {
    node.x >= 0 && node.x < w && node.y >= 0 && node.y < h
}

fn is_unique(node: &Node, map: &Vec<Vec<char>>) -> bool {
    map[node.y as usize][node.x as usize] != '#'
}

fn read_file(path:String)->Vec<Vec<char>>{
    let mut input= fs::read_to_string(path).expect("Could not read file");

    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}


fn populate_node_map(node_map: &mut HashMap<char, Vec<Node>>, map: &[Vec<char>], width: i32, height: i32) {
    for row_index in 0..height {
        for col_index in 0..width {
            let freq = map[row_index as usize][col_index as usize];
            if freq != '.' {
                let node = Node {
                    x: col_index as i32,
                    y: row_index as i32,
                };
                node_map.entry(freq).or_default().push(node);
            }
        }
    }
}

fn process_nodes_1(nodes: &[Node], map: &mut Vec<Vec<char>>, width: i32, height: i32) -> usize {
    let mut count = 0;

    for i in nodes {
        for j in nodes {
            if i == j {
                continue;
            }

            let (anti_a, anti_b) = extend_1(i, j);
            if inside_1(&anti_a, width, height) && is_unique(&anti_a, map) {
                count += 1;
                map[anti_a.y as usize][anti_a.x as usize] = '#';
            }
            if inside_1(&anti_b, width, height) && is_unique(&anti_b, map) {
                count += 1;
                map[anti_b.y as usize][anti_b.x as usize] = '#';
            }
        }
    }

    count
}

fn process_nodes_2(nodes: &[Node], map: &mut Vec<Vec<char>>, width: i32, height: i32) -> usize {
    let mut count = 0;

    for i in nodes {
        for j in nodes {
            if i == j {
                continue;
            }
            let anti_nodes = extend_2(i, j, width, height);
            for anti_node in anti_nodes {
                if is_unique(&anti_node, &map) {
                    count += 1;
                    map[anti_node.y as usize][anti_node.x as usize] = '#';
                }
            }
        }
    }
    count
}

fn print_map(map: &[Vec<char>]) {
    for row in map.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}