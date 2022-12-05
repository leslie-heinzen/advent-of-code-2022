use std::str::Lines;

use crate::shared::{has_partial_overlap, solve};

pub fn solve_part2(lines: Lines) -> usize {
    return solve(lines, has_partial_overlap);
}
