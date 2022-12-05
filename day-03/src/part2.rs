use std::{fs, collections::HashSet};

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let sum: _ = binding
        .lines()
        .chunks(3)
        .map(|l| {
            let (left, right) = l.as_bytes().split_at(split_idx);

            let left_compartment_priorities = get_compartment_priorities(left);
            let right_compartment_priorities = get_compartment_priorities(right);
            
            let mut intersection = left_compartment_priorities
                .intersection(&right_compartment_priorities);

            return *intersection.next().unwrap() as u32;

        })
        .sum::<u32>();

    println!("Part 1 Solution: {:#?}", sum);
}

fn get_compartment_priorities(slice: &[u8]) -> HashSet<u8> {
    return HashSet::from_iter(slice
        .iter()
        .map(|b| {
            if b >= &b'a' {
                return b - b'a' + 1;
            } else {
                return b - b'A' + 27;
            }
        }));
}