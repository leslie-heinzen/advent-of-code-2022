use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let binding = fs::read_to_string("input.txt").expect("can read file to string");
    let tree_rows: Vec<Vec<u32>> = binding
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total: u32 = 0;
    let mut scenic_scores = vec![];

    for (i, tree_row) in tree_rows.iter().enumerate() {
        for (j, tree_height) in tree_row.iter().enumerate() {
            if i == 0 || j == 0 || i == tree_rows.len() - 1 || j == tree_row.len() - 1 {
                total += 1;
                continue;
            }

            let neighbors = vec![
                check_neighbors(&tree_rows, i, j, Direction::Up, *tree_height, 1),
                check_neighbors(&tree_rows, i, j, Direction::Left, *tree_height, 1),
                check_neighbors(&tree_rows, i, j, Direction::Down, *tree_height, 1),
                check_neighbors(&tree_rows, i, j, Direction::Right, *tree_height, 1),
            ];

            if neighbors.iter().any(|&x| x.0) {
                total += 1;
            }

            let score: u32 = neighbors.iter().map(|n| n.1).into_iter().product();
            scenic_scores.push(score);
        }
    }

    println!("{}", total);
    println!("{}", scenic_scores.iter().max().unwrap());
}

// recursively check neighbors in a given direction
fn check_neighbors(
    tree_rows: &Vec<Vec<u32>>,
    i: usize,
    j: usize,
    direction: Direction,
    tree_height: u32,
    viewing_distance: u32,
) -> (bool, u32) {
    match direction {
        Direction::Up => {
            if i == 0 {
                return (true, viewing_distance - 1);
            }
            if tree_rows[i - 1][j] < tree_height {
                return check_neighbors(
                    tree_rows,
                    i - 1,
                    j,
                    Direction::Up,
                    tree_height,
                    viewing_distance + 1,
                );
            }
        }
        Direction::Down => {
            if i == tree_rows.len() - 1 {
                return (true, viewing_distance - 1);
            }
            if tree_rows[i + 1][j] < tree_height {
                return check_neighbors(
                    tree_rows,
                    i + 1,
                    j,
                    Direction::Down,
                    tree_height,
                    viewing_distance + 1,
                );
            }
        }
        Direction::Left => {
            if j == 0 {
                return (true, viewing_distance - 1);
            }
            if tree_rows[i][j - 1] < tree_height {
                return check_neighbors(
                    tree_rows,
                    i,
                    j - 1,
                    Direction::Left,
                    tree_height,
                    viewing_distance + 1,
                );
            }
        }
        Direction::Right => {
            if j == tree_rows[i].len() - 1 {
                return (true, viewing_distance - 1);
            }
            if tree_rows[i][j + 1] < tree_height {
                return check_neighbors(
                    tree_rows,
                    i,
                    j + 1,
                    Direction::Right,
                    tree_height,
                    viewing_distance + 1,
                );
            }
        }
    }

    return (false, viewing_distance);
}
