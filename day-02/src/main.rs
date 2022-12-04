use std::fs;


fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines = binding.lines();

    let total_scores = lines.map(|line| {
        let bytes = line.as_bytes();
        let opponent_move = (bytes[0] - b'A') as i8;
        let my_move = (bytes[2] - b'X') as i8;

        return (get_score_for_round_p1(my_move, opponent_move), 
            get_score_for_round_p2(my_move, opponent_move));
    })
    .reduce(|prev, curr| (prev.0 + curr.0, prev.1 + curr.1));

    println!("Part 1: {:#?}", total_scores.unwrap().0);
    println!("Part 2: {:#?}", total_scores.unwrap().1);
}

fn get_score_for_round_p1(my_move: i8, opponent_move: i8) -> i32 {
    let points = (my_move - opponent_move + 1).rem_euclid(3);
    let move_points = points * 3;
    let shape_points = my_move + 1;
    return (move_points + shape_points) as i32;
}

fn get_score_for_round_p2(my_move: i8, opponent_move: i8) -> i32 {
    let move_points = my_move * 3;
    let shape_points = (opponent_move - 1 + my_move).rem_euclid(3) + 1;
    return (move_points + shape_points) as i32;
}