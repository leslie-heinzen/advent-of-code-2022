use std::{collections::HashSet, fs};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let sum: _ = binding
        .lines()
        .map(|l| {
            let split_idx = l.len() / 2;
            let (left, right) = l.as_bytes().split_at(split_idx);

            let left_compartment_priorities = get_compartment_priorities(left);
            let right_compartment_priorities = get_compartment_priorities(right);

            let mut intersection =
                left_compartment_priorities.intersection(&right_compartment_priorities);

            return *intersection.next().unwrap() as u32;
        })
        .sum::<u32>();

    println!("Part 1 Solution: {:#?}", sum);
}

fn part_2() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines_vec: Vec<&str> = binding.lines().collect();
    let sum: _ = lines_vec
        .chunks(3)
        .map(|chunk| {
            let group_rucksacks: Vec<HashSet<u8>> = chunk
                .iter()
                .map(|rucksack| {
                    let bytes = rucksack.as_bytes();
                    let compartment_priorities = get_compartment_priorities(bytes);
                    return compartment_priorities;
                })
                .collect();

            let iter = group_rucksacks.iter();
            let group_intersection = iter.skip(1).fold(group_rucksacks[0].clone(), |set1, set2| {
                set1.intersection(set2).cloned().collect()
            });

            return *group_intersection.iter().next().unwrap() as u32;
        })
        .sum::<u32>();

    println!("Part 2 Solution: {:#?}", sum);
}

fn get_compartment_priorities(slice: &[u8]) -> HashSet<u8> {
    return HashSet::from_iter(slice.iter().map(|b| {
        if b >= &b'a' {
            return b - b'a' + 1;
        } else {
            return b - b'A' + 27;
        }
    }));
}
