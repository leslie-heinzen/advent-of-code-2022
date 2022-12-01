use std::fs;

fn main() {
    let input_string = fs::read_to_string("input.txt")
        .expect("should read file to string");

    let solution_1 = part_1(sum_elf_calorie_totals(&input_string));
    let solution_2 = part_2(sum_elf_calorie_totals(&input_string));
    println!("Sln 1: {}", solution_1);
    println!("Sln 2: {}", solution_2);
}

fn sum_elf_calorie_totals(input: &String) -> Vec<u32> {
    let mut elf_calorie_totals: Vec<u32> = vec![];

    for segment in input.split("\n\n") {
        let elf_total = segment.split("\n")
            .map(|s| s.parse::<u32>().unwrap())
            .sum::<u32>();
        elf_calorie_totals.push(elf_total);
    }

    return elf_calorie_totals;
}

fn part_1(input: Vec<u32>) -> u32 {
    let max = input.iter().max().expect("max has value");
    return *max;
}

fn part_2(mut input: Vec<u32>) -> u32 {
    input.sort();
    let top_three_sum: u32 = input.iter().rev().take(3).sum();
    return top_three_sum;
}
