use std::str::Lines;

pub trait ToBoundary {
    fn to_boundary_tuple(&self) -> (u32, u32);
}

impl ToBoundary for str {
    fn to_boundary_tuple(&self) -> (u32, u32) {
        let (start_str, end_str) = self.split_once("-").unwrap();

        let start = start_str.parse::<u32>().unwrap();
        let end = end_str.parse::<u32>().unwrap();

        return (start, end);
    }
}

pub fn solve(lines: Lines, overlap_fn: fn(a: (u32, u32), b: (u32, u32)) -> bool) -> usize {
    return lines
        .map(|line| {
            let (left_str, right_str) = line.split_once(",").unwrap();
            let first_elf_bounds = left_str.to_boundary_tuple();
            let second_elf_bounds = right_str.to_boundary_tuple();

            return overlap_fn(first_elf_bounds, second_elf_bounds);
        })
        .filter(|r| *r == true)
        .count();
}

pub fn has_total_overlap(tuple_a: (u32, u32), tuple_b: (u32, u32)) -> bool {
    let a_contains_b = tuple_a.0 >= tuple_b.0 && tuple_a.1 <= tuple_b.1;
    let b_contains_a = tuple_b.0 >= tuple_a.0 && tuple_b.1 <= tuple_a.1;

    if a_contains_b || b_contains_a {
        return true;
    }

    return false;
}

pub fn has_partial_overlap(tuple_a: (u32, u32), tuple_b: (u32, u32)) -> bool {
    let a_end_overlaps_b_start = tuple_a.0 <= tuple_b.0 && tuple_b.0 <= tuple_a.1;
    let b_end_overlaps_a_start = tuple_b.0 <= tuple_a.0 && tuple_a.0 <= tuple_b.1;

    if has_total_overlap(tuple_a, tuple_b) {
        return true;
    } else if a_end_overlaps_b_start || b_end_overlaps_a_start {
        return true;
    }

    return false;
}
