use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day17/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

fn populate_matrix(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Matrix = Vec<Vec<u32>>;

type Position = (usize, usize);
type StraightLineCount = u8;
type AccumulatedCost = u32;

type Map = HashMap<(usize, usize, Direction, StraightLineCount), AccumulatedCost>;

type QueueItem = (Position, Direction, StraightLineCount, AccumulatedCost);

fn is_bottom_right(matrix: &Matrix, position: Position) -> bool {
    position == (matrix.len() - 1, matrix[0].len() - 1)
}

fn is_able_to_go(matrix: &Matrix, current_node: &QueueItem, to: Direction) -> bool {
    let (row, col) = current_node.0;
    let from = &current_node.1;
    let straight_line_count = current_node.2;

    match to {
        Direction::North => {
            if *from == Direction::North
                || row == 0
                || (*from == Direction::South && straight_line_count >= 10)
                || ((*from == Direction::East || *from == Direction::West)
                    && straight_line_count < 4)
            {
                return false;
            }
        }
        Direction::South => {
            if *from == Direction::South
                || row == matrix.len() - 1
                || (*from == Direction::North && straight_line_count >= 10)
                || ((*from == Direction::East || *from == Direction::West)
                    && straight_line_count < 4)
            {
                return false;
            }
        }
        Direction::East => {
            if *from == Direction::East
                || col == matrix[0].len() - 1
                || (*from == Direction::West && straight_line_count >= 10)
                || ((*from == Direction::North || *from == Direction::South)
                    && straight_line_count < 4)
            {
                return false;
            }
        }
        Direction::West => {
            if *from == Direction::West
                || col == 0
                || (*from == Direction::East && straight_line_count >= 10)
                || ((*from == Direction::North || *from == Direction::South)
                    && straight_line_count < 4)
            {
                return false;
            }
        }
    }
    true
}

fn add_node_to_next_iteration(
    visited: &mut Map,
    queue: &mut VecDeque<QueueItem>,
    position: &Position,
    direction_from: Direction,
    straight_line_count: StraightLineCount,
    accumulated_cost: AccumulatedCost,
) {
    let (row, col) = *position;
    if let Some(accumulated_cost_of_visited_node) =
        visited.get(&(row, col, direction_from.clone(), straight_line_count))
    {
        if accumulated_cost < *accumulated_cost_of_visited_node {
            visited.insert(
                (row, col, direction_from.clone(), straight_line_count),
                accumulated_cost,
            );
            queue.push_back((
                (row, col),
                direction_from,
                straight_line_count,
                accumulated_cost,
            ));
        }
    } else {
        visited.insert(
            (row, col, direction_from.clone(), straight_line_count),
            accumulated_cost,
        );
        queue.push_back((
            (row, col),
            direction_from,
            straight_line_count,
            accumulated_cost,
        ));
    }
}

fn visit(
    matrix: &Matrix,
    queue: &mut VecDeque<QueueItem>,
    visited: &mut Map,
    min_cost: &mut u32,
    current_node: QueueItem,
) {
    let current_position = current_node.0;

    if is_bottom_right(matrix, current_position) && current_node.2 >= 4 {
        if current_node.3 < *min_cost {
            *min_cost = current_node.3;
        }
        return;
    }

    if is_able_to_go(matrix, &current_node, Direction::North) {
        let from = &current_node.1;

        let north_straight_line_count = match from {
            Direction::East | Direction::West => 1,
            Direction::South => current_node.2 + 1,
            _ => 0,
        };

        add_node_to_next_iteration(
            visited,
            queue,
            &(current_position.0 - 1, current_position.1),
            Direction::South,
            north_straight_line_count,
            current_node.3 + matrix[current_position.0 - 1][current_position.1],
        );
    }

    if is_able_to_go(matrix, &current_node, Direction::South) {
        let from = &current_node.1;

        let south_straight_line_count = match from {
            Direction::East | Direction::West => 1,
            Direction::North => current_node.2 + 1,
            _ => 0,
        };

        add_node_to_next_iteration(
            visited,
            queue,
            &(current_position.0 + 1, current_position.1),
            Direction::North,
            south_straight_line_count,
            current_node.3 + matrix[current_position.0 + 1][current_position.1],
        );
    }

    if is_able_to_go(matrix, &current_node, Direction::East) {
        let from = &current_node.1;

        let east_straight_line_count = match from {
            Direction::North | Direction::South => 1,
            Direction::West => current_node.2 + 1,
            _ => 0,
        };

        add_node_to_next_iteration(
            visited,
            queue,
            &(current_position.0, current_position.1 + 1),
            Direction::West,
            east_straight_line_count,
            current_node.3 + matrix[current_position.0][current_position.1 + 1],
        );
    }

    if is_able_to_go(matrix, &current_node, Direction::West) {
        let from = &current_node.1;

        let west_straight_line_count = match from {
            Direction::North | Direction::South => 1,
            Direction::East => current_node.2 + 1,
            _ => 0,
        };

        add_node_to_next_iteration(
            visited,
            queue,
            &(current_position.0, current_position.1 - 1),
            Direction::East,
            west_straight_line_count,
            current_node.3 + matrix[current_position.0][current_position.1 - 1],
        );
    }
}

fn solution(input: &str) -> u32 {
    let matrix = populate_matrix(input);
    let mut queue: VecDeque<QueueItem> = VecDeque::new();
    queue.push_back(((0, 1), Direction::West, 1, matrix[0][1]));
    queue.push_back(((1, 0), Direction::North, 1, matrix[1][0]));

    let mut visited: Map = HashMap::new();
    let mut min_cost = u32::MAX;

    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();

        visit(
            &matrix,
            &mut queue,
            &mut visited,
            &mut min_cost,
            current_node,
        );
    }
    min_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_populate_matrix() {
        let input = "123\n456\n789";

        assert_eq!(
            populate_matrix(input),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],]
        )
    }

    #[test]
    fn it_can_check_if_is_bottom_right() {
        let matrix = vec![
            vec![2, 4], //
            vec![3, 2], //
        ];

        assert_eq!(is_bottom_right(&matrix, (1, 1)), true);
        assert_eq!(is_bottom_right(&matrix, (0, 0)), false);
    }

    #[test]
    fn it_can_check_if_is_able_to_go() {
        let matrix = vec![
            vec![2, 4, 1], //
            vec![3, 2, 4], //
            vec![5, 8, 1], //
        ];

        assert_eq!(
            is_able_to_go(&matrix, &((0, 1), Direction::West, 1, 4), Direction::North),
            false
        );
        assert_eq!(
            is_able_to_go(&matrix, &((0, 1), Direction::West, 1, 4), Direction::South),
            false
        );
        assert_eq!(
            is_able_to_go(&matrix, &((0, 1), Direction::West, 1, 4), Direction::East),
            true
        );
        assert_eq!(
            is_able_to_go(&matrix, &((0, 1), Direction::West, 1, 4), Direction::West),
            false
        );

        assert_eq!(
            is_able_to_go(&matrix, &((0, 2), Direction::West, 2, 5), Direction::South),
            false
        );

        let matrix = vec![
            vec![2, 4, 1, 3, 1], //
            vec![3, 2, 4, 4, 2], //
            vec![5, 8, 1, 5, 3], //
            vec![9, 7, 6, 6, 4], //
            vec![2, 8, 3, 7, 5], //
            vec![5, 8, 0, 3, 2], //
        ];

        assert_eq!(
            is_able_to_go(&matrix, &((0, 3), Direction::West, 3, 8), Direction::South),
            false
        );
        assert_eq!(
            is_able_to_go(&matrix, &((0, 4), Direction::West, 4, 9), Direction::South),
            true
        );
        assert_eq!(
            is_able_to_go(&matrix, &((5, 0), Direction::North, 5, 24), Direction::East),
            true
        );
        let matrix = vec![
            vec![2, 4, 1, 3, 1], //
            vec![3, 2, 4, 4, 2], //
            vec![5, 8, 1, 5, 3], //
            vec![9, 7, 6, 6, 4], //
            vec![2, 8, 3, 7, 5], //
            vec![5, 8, 0, 3, 2], //
            vec![3, 2, 4, 4, 2], //
            vec![5, 8, 1, 5, 3], //
            vec![9, 7, 6, 6, 4], //
            vec![2, 4, 1, 3, 1], //
            vec![3, 2, 4, 4, 2], //
            vec![2, 8, 3, 7, 5], //
            vec![5, 8, 0, 3, 2], //
            vec![3, 2, 4, 4, 2], //
        ];

        assert_eq!(
            is_able_to_go(
                &matrix,
                &((9, 1), Direction::North, 9, 58),
                Direction::South
            ),
            true
        );
        assert_eq!(
            is_able_to_go(
                &matrix,
                &((10, 1), Direction::North, 10, 60),
                Direction::South
            ),
            false
        );
    }

    #[test]
    fn it_can_add_node_to_next_iteration() {
        let mut visited: Map = HashMap::new();
        let mut queue: VecDeque<QueueItem> = VecDeque::new();

        // let current_node: QueueItem = ((3, 1), Direction::North, 3, 21);
        // let west_from_current: QueueItem = ((3, 0), Direction::East, 1, 30);
        // let east_from_current: QueueItem = ((3, 2), Direction::West, 1, 27);

        add_node_to_next_iteration(&mut visited, &mut queue, &(3, 0), Direction::East, 1, 30);
        add_node_to_next_iteration(&mut visited, &mut queue, &(3, 2), Direction::West, 1, 27);

        assert_eq!(queue.len(), 2);
        assert_eq!(visited.get(&(3, 0, Direction::East, 1)), Some(&30));
    }
}
