use std::{collections::HashSet, fs};

fn main() {
    let input_chars: Vec<char> = fs::read_to_string("input.txt")
        .expect("should read file to string")
        .chars()
        .collect();
    println!("Solution 1: {count}", count = solve(&input_chars, 4));
    println!("Solution 2: {count}", count = solve(&input_chars, 14));
}

fn solve(chars: &Vec<char>, sequence_len: usize) -> usize {
    for i in 0..chars.len() {
        let char_count = i + sequence_len;
        let segment: Vec<&char> = chars[i..char_count].iter().collect();

        if HashSet::<&char>::from_iter(segment).len() == sequence_len {
            return char_count;
        }
    }

    0
}
