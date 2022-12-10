use std::{fs, collections::VecDeque};

fn main() {
    let binding = fs::read_to_string("input.txt").expect("Unable to read file");
    let commands: Vec<Vec<&str>> = binding
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect();

    let mut register_value = 1;
    let mut tick: usize = 0;
    let mut instructions: VecDeque<(i32, usize)> = VecDeque::new();


    for command in &commands {
        let (_, last_tick_to_execute) = instructions.back().cloned().unwrap_or((0, 0));

        match command[0] {
            "noop" => {
                instructions.push_back((0, last_tick_to_execute + 1));
            }
            "addx" => {
                let value = command[1].parse::<i32>().unwrap();
                instructions.push_back((value, last_tick_to_execute + 2));
                println!("Add instruction: {value} at {idx}", value = value, idx = last_tick_to_execute + 2);
            }
            _ => panic!("Unknown command"),
        }
    }


    let mut signal_strengths: Vec<i32> = vec![];
    loop {
        if let Some((value, execution_tick)) = instructions.get(0).cloned() {
            if execution_tick == tick {
                println!("Executing instruction: {value} at {tick}");
                register_value += value;
                instructions.pop_front();
            }
        }

        tick += 1;

        if tick == 20 || tick % 40 == 20 {
            let signal_strength = tick as i32 * register_value;
            signal_strengths.push(signal_strength);
            println!("Signal strength at {tick}: {signal_strength}");
        }

        if instructions.is_empty() {
            break;
        }
    }

    println!("Part 1 Solution: {signal_strengths:?}", signal_strengths = signal_strengths.iter().sum::<i32>());
}
