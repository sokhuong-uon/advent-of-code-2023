use crate::part1::solution::{
    find_cordinate_of_pipes_connected_to_start_position, get_next_position, populate_matrix,
};
use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day10/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

fn mark_pipe_as_edge(matrix: &mut Matrix, position: Position) {
    let char = matrix[position.0][position.1];
    if char == '|' || char == 'J' || char == 'L' {
        matrix[position.0][position.1] = 'N';
    } else {
        matrix[position.0][position.1] = 'E';
    }
}

fn mark_edges(
    matrix: &mut Matrix,
    start_position: Position,
    cordinate1: &mut Position,
    cordinate2: &mut Position,
) {
    mark_pipe_as_edge(matrix, start_position);

    let mut previous_position1 = start_position;
    let mut previous_position2 = start_position;

    while cordinate1 != cordinate2 {
        let next_position1 = get_next_position(&matrix, *cordinate1, previous_position1);
        let next_position2 = get_next_position(&matrix, *cordinate2, previous_position2);

        previous_position1 = *cordinate1;
        previous_position2 = *cordinate2;
        mark_pipe_as_edge(matrix, previous_position1);
        mark_pipe_as_edge(matrix, previous_position2);

        *cordinate1 = next_position1;
        *cordinate2 = next_position2;
    }
    mark_pipe_as_edge(matrix, *cordinate1);
}

pub type Matrix = Vec<Vec<char>>;
pub type Position = (usize, usize);

fn solution(input: &str) -> i64 {
    let mut total = 0;

    let mut matrix: Matrix = Vec::new();
    let start_position = populate_matrix(&mut matrix, input);
    let (mut cordinate1, mut cordinate2) =
        find_cordinate_of_pipes_connected_to_start_position(&mut matrix, start_position);

    mark_edges(
        &mut matrix,
        start_position,
        &mut cordinate1,
        &mut cordinate2,
    );

    let mut is_inside_maze = false;

    for row in matrix.iter() {
        for char in row.iter() {
            if *char == 'N' {
                is_inside_maze = !is_inside_maze;
            } else if *char != 'E' {
                if is_inside_maze {
                    total += 1;
                }
            }
        }
        is_inside_maze = false;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_mark_edges() {
        let mut matrix = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'F', '-', '-', '-', '-', '-', '-', '-', '7', '.'],
            vec!['.', '|', 'F', '-', '-', '-', '-', '-', '7', '|', '.'],
            vec!['.', '|', '|', '.', '.', '.', '.', '.', '|', '|', '.'],
            vec!['.', '|', '|', '.', '.', '.', '.', '.', '|', '|', '.'],
            vec!['.', '|', 'L', '-', '7', '.', 'F', '-', 'J', '|', '.'],
            vec!['.', '|', '.', '.', '|', '.', '|', '.', '.', '|', '.'],
            vec!['.', 'L', '-', '-', 'J', '.', 'L', '-', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let start_position = (1, 1);
        let (mut cordinate1, mut cordinate2) =
            find_cordinate_of_pipes_connected_to_start_position(&mut matrix, start_position);

        mark_edges(
            &mut matrix,
            start_position,
            &mut cordinate1,
            &mut cordinate2,
        );

        let expected = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', '.'],
            vec!['.', 'N', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'N', '.'],
            vec!['.', 'N', 'N', '.', '.', '.', '.', '.', 'N', 'N', '.'],
            vec!['.', 'N', 'N', '.', '.', '.', '.', '.', 'N', 'N', '.'],
            vec!['.', 'N', 'N', 'E', 'E', '.', 'E', 'E', 'N', 'N', '.'],
            vec!['.', 'N', '.', '.', 'N', '.', 'N', '.', '.', 'N', '.'],
            vec!['.', 'N', 'E', 'E', 'N', '.', 'N', 'E', 'E', 'N', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        assert_eq!(matrix, expected);
    }
}
