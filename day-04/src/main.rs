use std::fs;

pub mod part1;
pub mod part2;
pub mod shared;

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let part1_solution = part1::solve_part1(binding.lines());
    let part2_solution = part2::solve_part2(binding.lines());

    println!("Part 1 solution: {part1_solution}");
    println!("Part 2 solution: {part2_solution}");
}
