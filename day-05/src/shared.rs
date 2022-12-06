use regex::Regex;
use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

pub fn solve(is_part_1: bool) -> String {
    // gen hashmap
    let mut crate_positions_hashmap = generate_pos_hashmap();

    // parse procedure
    let procedure_instructions = parse_procedure();

    // execute crate moves
    crate_positions_hashmap = execute_crate_moves(
        procedure_instructions,
        crate_positions_hashmap.clone(),
        is_part_1,
    );

    let message = create_message(crate_positions_hashmap);

    return message;
}

fn generate_pos_hashmap() -> HashMap<String, Vec<char>> {
    let binding = fs::read_to_string("input/start-positions.txt").expect("should read to string");
    let mut pos_lines: Vec<Vec<char>> = binding
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut crate_positions_hashmap = HashMap::new();
    let stack_numbers = pos_lines
        .pop()
        .expect("should return a value from pos_lines");

    for line in pos_lines.iter().rev() {
        for y in 0..line.len() {
            if (y as i16 - 1).rem_euclid(4) == 0 {
                let content = line[y];

                if content.is_whitespace() {
                    continue;
                }

                let key = stack_numbers[y].to_string();

                match crate_positions_hashmap.entry(key) {
                    Entry::Vacant(e) => {
                        e.insert(vec![content]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(content);
                    }
                }
            }
        }
    }

    crate_positions_hashmap
}

fn parse_procedure() -> Vec<Vec<String>> {
    let binding = fs::read_to_string("input/procedure.txt").expect("should read to string");
    let instructions: Vec<Vec<String>> = binding
        .lines()
        .map(|l| {
            let matches: Vec<String> = (Regex::new(r"\d{1,3}"))
                .expect("should compile a regex")
                .find_iter(l)
                .map(|x| x.as_str().to_string())
                .collect();

            return matches.to_owned();
        })
        .collect();

    return instructions;
}

fn execute_crate_moves(
    instructions: Vec<Vec<String>>,
    mut crate_positions_hashmap: HashMap<String, Vec<char>>,
    is_part_1: bool,
) -> HashMap<String, Vec<char>> {
    for instruction in instructions {
        let num_of_crates = instruction[0]
            .parse::<u32>()
            .expect("should parse a u32 int");
        let from_stack = instruction[1].clone();
        let to_stack = instruction[2].clone();

        if is_part_1 {
            crate_positions_hashmap =
                move_single_crates(crate_positions_hashmap, num_of_crates, from_stack, to_stack);
        } else {
            crate_positions_hashmap =
                move_bulk_crates(crate_positions_hashmap, num_of_crates, from_stack, to_stack);
        }
    }

    return crate_positions_hashmap;
}

fn move_single_crates(
    mut crate_positions_hashmap: HashMap<String, Vec<char>>,
    num_of_crates: u32,
    from_stack: String,
    to_stack: String,
) -> HashMap<String, Vec<char>> {
    for _ in 0..num_of_crates {
        let val = crate_positions_hashmap
            .get_mut(&from_stack)
            .expect("should return a mutable ref for the hashmap")
            .pop();

        crate_positions_hashmap
            .get_mut(&to_stack)
            .expect("should return a mutable ref for the hashmap")
            .push(val.unwrap());
    }

    return crate_positions_hashmap;
}

fn move_bulk_crates(
    mut crate_positions_hashmap: HashMap<String, Vec<char>>,
    num_of_crates: u32,
    from_stack: String,
    to_stack: String,
) -> HashMap<String, Vec<char>> {
    let mut moved_crates = vec![];

    for _ in 0..num_of_crates {
        let val = crate_positions_hashmap
            .get_mut(&from_stack)
            .expect("should return a mutable ref for the hashmap")
            .pop();

        moved_crates.push(val.unwrap());
    }

    moved_crates.reverse();

    for mc in moved_crates {
        crate_positions_hashmap
            .get_mut(&to_stack)
            .expect("should return a mutable ref for the hashmap")
            .push(mc);
    }

    return crate_positions_hashmap;
}

fn create_message(mut crate_positions_hashmap: HashMap<String, Vec<char>>) -> String {
    let mut results: Vec<String> = vec![];
    for i in 0..=crate_positions_hashmap.len() {
        match crate_positions_hashmap.entry(i.to_string()) {
            Entry::Occupied(mut e) => {
                let val = e.get_mut().pop().unwrap().to_string();
                results.push(val);
            }
            Entry::Vacant(_) => {}
        }
    }

    return results.join("");
}
