use std::{fs, collections::HashSet};

fn main() {
    let binding = fs::read_to_string("example.txt").unwrap();
    let lines: Vec<(&str, &str)> = binding
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .collect();

    let mut visited = HashSet::<(i32, i32)>::new();
    let (mut h_x, mut h_y) = (0, 0);
    let (mut t_x, mut t_y) = (0, 0);

    for (direction, distance) in lines {
        println!("{} {}", direction, distance);
        let distance = distance.parse::<u32>().unwrap();
        for _ in 0..distance {            
            match direction {
                "R" => {
                    h_x += 1;
                    if h_x - t_x > 1 {
                        t_y = h_y;
                        t_x += 1;
                    }
                }
                "L" => {
                    h_x -= 1;
                    if t_x - h_x > 1 {
                        t_y = h_y;
                        t_x -= 1;
                    }
                }
                "U" => {
                    h_y += 1;
                    if h_y - t_y > 1 {
                        t_x = h_x;
                        t_y += 1;
                    }
                }
                "D" => {
                    h_y -= 1;
                    if t_y - h_y > 1 {
                        t_x = h_x;
                        t_y -= 1;
                    }
                }
                _ => panic!("Unknown direction"),
            }
            visited.insert((t_x, t_y));
        }
    }

    println!("Visited: {}", visited.len());
}
