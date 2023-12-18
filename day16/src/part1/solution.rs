use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day16/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

type Matrix = Vec<Vec<char>>;
type Position = (usize, usize);
type Map = HashMap<Position, Vec<Direction>>;

fn populate_matrix(input: &str) -> Matrix {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn is_able_to_go(matrix: &Matrix, current_position: &Position, direction: &Direction) -> bool {
    let max_y = matrix.len() - 1;
    let max_x = matrix[0].len() - 1;

    match direction {
        Direction::North => current_position.0 > 0,
        Direction::South => current_position.0 < max_y,
        Direction::East => current_position.1 < max_x,
        Direction::West => current_position.1 > 0,
    }
}

fn visit(matrix: &Matrix, visited: &mut Map, current_position: &Position, from: Direction) {
    let mut is_visited = false;
    let mut is_visited_from_different_direction = false;

    if let Some(directions) = visited.get_mut(current_position) {
        is_visited = true;

        let current_direction = from.clone();

        if directions.contains(&from) {
            return;
        } else {
            directions.push(current_direction);
            is_visited_from_different_direction = true;
        }
    }

    if !is_visited || is_visited_from_different_direction {
        visited.insert(current_position.clone(), vec![from.clone()]);

        match matrix[current_position.0][current_position.1] {
            '|' => match from {
                Direction::North => {
                    if is_able_to_go(matrix, current_position, &Direction::South) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0 + 1, current_position.1),
                            Direction::North,
                        )
                    }
                }
                Direction::South => {
                    if is_able_to_go(matrix, current_position, &Direction::North) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0 - 1, current_position.1),
                            Direction::South,
                        )
                    }
                }
                Direction::East | Direction::West => {
                    if is_able_to_go(matrix, current_position, &Direction::North) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0 - 1, current_position.1),
                            Direction::South,
                        )
                    }

                    if is_able_to_go(matrix, current_position, &Direction::South) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0 + 1, current_position.1),
                            Direction::North,
                        )
                    }
                }
            },
            '-' => match from {
                Direction::East => {
                    if is_able_to_go(matrix, current_position, &Direction::West) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0, current_position.1 - 1),
                            Direction::East,
                        )
                    }
                }
                Direction::West => {
                    if is_able_to_go(matrix, current_position, &Direction::East) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0, current_position.1 + 1),
                            Direction::West,
                        )
                    }
                }
                Direction::North | Direction::South => {
                    if is_able_to_go(matrix, current_position, &Direction::East) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0, current_position.1 + 1),
                            Direction::West,
                        )
                    }

                    if is_able_to_go(matrix, current_position, &Direction::West) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0, current_position.1 - 1),
                            Direction::East,
                        )
                    }
                }
            },
            '\\' => match from {
                Direction::East => {
                    if is_able_to_go(matrix, current_position, &Direction::North) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0 - 1, current_position.1),
                            Direction::South,
                        )
                    }
                }
                Direction::North => {
                    if is_able_to_go(matrix, current_position, &Direction::East) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0, current_position.1 + 1),
                            Direction::West,
                        )
                    }
                }
                Direction::South => {
                    if is_able_to_go(matrix, current_position, &Direction::West) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0, current_position.1 - 1),
                            Direction::East,
                        )
                    }
                }
                Direction::West => {
                    if is_able_to_go(matrix, current_position, &Direction::South) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0 + 1, current_position.1),
                            Direction::North,
                        )
                    }
                }
            },
            '/' => match from {
                Direction::East => {
                    if is_able_to_go(matrix, current_position, &Direction::South) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0 + 1, current_position.1),
                            Direction::North,
                        )
                    }
                }
                Direction::South => {
                    if is_able_to_go(matrix, current_position, &Direction::East) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0, current_position.1 + 1),
                            Direction::West,
                        )
                    }
                }
                Direction::North => {
                    if is_able_to_go(matrix, current_position, &Direction::West) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0, current_position.1 - 1),
                            Direction::East,
                        )
                    }
                }
                Direction::West => {
                    if is_able_to_go(matrix, current_position, &Direction::North) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0 - 1, current_position.1),
                            Direction::South,
                        )
                    }
                }
            },
            '.' => match from {
                Direction::East => {
                    if is_able_to_go(matrix, current_position, &Direction::West) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0, current_position.1 - 1),
                            Direction::East,
                        )
                    }
                }
                Direction::West => {
                    if is_able_to_go(matrix, current_position, &Direction::East) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0, current_position.1 + 1),
                            Direction::West,
                        )
                    }
                }
                Direction::North => {
                    if is_able_to_go(matrix, current_position, &Direction::South) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0 + 1, current_position.1),
                            Direction::North,
                        )
                    }
                }
                Direction::South => {
                    if is_able_to_go(matrix, current_position, &Direction::North) {
                        visit(
                            matrix,
                            visited,
                            &(current_position.0 - 1, current_position.1),
                            Direction::South,
                        )
                    }
                }
            },
            _ => {}
        }
    }
}

fn solution(input: &str) -> u32 {
    let matrix = populate_matrix(input);
    let mut visited: Map = HashMap::new();

    let start_position: Position = (0, 0);
    visit(&matrix, &mut visited, &start_position, Direction::West);

    visited.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_populate_matrix() {
        let input = r".|...\....
|.-.\.....";
        let matrix = populate_matrix(input);
        assert_eq!(matrix.len(), 2);
        assert_eq!(
            matrix,
            vec![
                vec!['.', '|', '.', '.', '.', '\\', '.', '.', '.', '.'],
                vec!['|', '.', '-', '.', '\\', '.', '.', '.', '.', '.']
            ]
        );
    }
}
