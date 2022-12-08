use crate::shared::Tree;
use std::fs;

pub mod shared;

const MAX_SIZE: usize = 100000;
const TOTAL_DISK_SIZE: usize = 70000000;
const REQUIRED_FREE_DISK_SPACE: usize = 30000000;

fn main() {
    let binding = fs::read_to_string("input.txt").expect("should read file to string");
    let lines = binding.lines();

    let mut tree = Tree { nodes: vec![] };
    tree.append_node(0, None);
    let mut current: Option<usize> = Some(0);

    for l in lines {
        let (left, right) = l.rsplit_once(" ").expect("msg");

        match left {
            "$ cd" => match right {
                "/" => {}
                ".." => {
                    let current_node = &tree.nodes[current.unwrap()];
                    current = current_node.parent_idx;
                }
                _ => {
                    tree.append_node(0, current);
                    current = tree.get_last_index();
                }
            },
            "$" => {} // ls
            "dir" => {}
            _ => {
                tree.append_node(left.parse::<usize>().unwrap(), current);
            }
        }
    }

    let part1_sum: usize = tree
        .nodes
        .iter()
        .filter(|n| n.children_idxs.len() > 0 && n.value <= MAX_SIZE)
        .map(|n| n.value)
        .sum();
    println!("Part 1 Solution: {part1_sum}");

    let current_free_disk_space = TOTAL_DISK_SIZE - tree.nodes[0].value;
    let min_deletion_size = REQUIRED_FREE_DISK_SPACE - current_free_disk_space;
    let mut part2_solution: Vec<usize> = tree
        .nodes
        .iter()
        .filter(|n| n.children_idxs.len() > 0 && n.value >= min_deletion_size)
        .map(|n| n.value)
        .collect();
    part2_solution.sort();
    println!("Part 2 solution: {first}", first = part2_solution[0]);
}
