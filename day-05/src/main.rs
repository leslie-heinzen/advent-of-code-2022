use part1::solve_part_1;
use part2::solve_part_2;

pub mod part1;
pub mod part2;
pub mod shared;

fn main() {
    println!("Part 1 Solution: {s}", s = solve_part_1());
    println!("Part 2 Solution: {s}", s = solve_part_2());
}
