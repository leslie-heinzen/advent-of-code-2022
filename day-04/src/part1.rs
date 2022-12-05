use std::str::Lines;

use crate::shared::{has_total_overlap, solve};

pub fn solve_part1(lines: Lines) -> usize {
    return solve(lines, has_total_overlap);
}
