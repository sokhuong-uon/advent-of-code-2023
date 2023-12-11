use crate::part1::solution::Matrix;
use crate::part1::solution::Position;

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub fn is_walkable(matrix: &Matrix, current_position: Position, direction: Direction) -> bool {
    match direction {
        Direction::North => is_north_walkable(matrix, current_position),
        Direction::South => is_south_walkable(matrix, current_position),
        Direction::East => is_east_walkable(matrix, current_position),
        Direction::West => is_west_walkable(matrix, current_position),
    }
}

fn is_north_walkable(matrix: &Matrix, current_position: Position) -> bool {
    if current_position.0 == 0 {
        false
    } else {
        let north_char = matrix[current_position.0 - 1][current_position.1];
        let current_char = matrix[current_position.0][current_position.1];

        match current_char {
            '7' | 'F' | '-' => false,
            'L' | '|' | 'J' => {
                if north_char == '|' || north_char == '7' || north_char == 'F' {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

fn is_south_walkable(matrix: &Matrix, current_position: Position) -> bool {
    if current_position.0 == matrix.len() - 1 {
        false
    } else {
        let south_char = matrix[current_position.0 + 1][current_position.1];
        let current_char = matrix[current_position.0][current_position.1];

        match current_char {
            'L' | 'J' | '-' => false,
            'F' | '|' | '7' => {
                if south_char == '|' || south_char == 'J' || south_char == 'L' {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

fn is_east_walkable(matrix: &Matrix, current_position: Position) -> bool {
    if current_position.1 == matrix[0].len() - 1 {
        false
    } else {
        let east_char = matrix[current_position.0][current_position.1 + 1];
        let current_char = matrix[current_position.0][current_position.1];

        match current_char {
            '7' | 'J' | '|' => false,
            'F' | 'L' | '-' => {
                if east_char == '-' || east_char == '7' || east_char == 'J' {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

fn is_west_walkable(matrix: &Matrix, current_position: Position) -> bool {
    if current_position.1 == 0 {
        false
    } else {
        let west_char = matrix[current_position.0][current_position.1 - 1];
        let current_char = matrix[current_position.0][current_position.1];

        match current_char {
            'L' | 'F' | '|' => false,
            '7' | '-' | 'J' => {
                if west_char == '-' || west_char == 'L' || west_char == 'F' {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_check_if_north_is_walkable() {
        let matrix = vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'F', '-', '7', '|'],
            vec!['L', '|', '7', '|', '|'],
            vec!['-', 'L', '-', 'J', 'F'],
            vec!['L', '|', '-', 'J', 'L'],
            vec!['L', '|', 'F', 'J', '7'],
            vec!['L', 'J', 'J', 'J', 'J'],
        ];

        // Outside of matrix
        let current_position = (0, 1);
        assert_eq!(is_north_walkable(&matrix, current_position), false);

        // Cannot walk north from 'F'
        let current_position = (1, 1);
        assert_eq!(is_north_walkable(&matrix, current_position), false);

        // Cannot walk north from '7'
        let current_position = (1, 3);
        assert_eq!(is_north_walkable(&matrix, current_position), false);

        // Cannot walk north from '-'
        let current_position = (1, 2);
        assert_eq!(is_north_walkable(&matrix, current_position), false);

        // Can walk north from 'L' to '7'
        let current_position = (2, 0);
        assert_eq!(is_north_walkable(&matrix, current_position), true);

        // Cann walk north from 'L' to '|'
        let current_position = (3, 1);
        assert_eq!(is_north_walkable(&matrix, current_position), true);

        // Can walk north from 'L' to 'F'
        let current_position = (4, 4);
        assert_eq!(is_north_walkable(&matrix, current_position), true);

        // Can walk north from '|' to '7'
        let current_position = (1, 4);
        assert_eq!(is_north_walkable(&matrix, current_position), true);

        // Can walk north from '|' to 'F'
        let current_position = (2, 1);
        assert_eq!(is_north_walkable(&matrix, current_position), true);

        // Can walk north from '|' to '|'
        let current_position = (2, 4);
        assert_eq!(is_north_walkable(&matrix, current_position), true);

        // Can walk north from 'J' to '|'
        let current_position = (3, 3);
        assert_eq!(is_north_walkable(&matrix, current_position), true);

        // Can walk north from 'J' to '7'
        let current_position = (6, 4);
        assert_eq!(is_north_walkable(&matrix, current_position), true);

        // Can walk north from 'J' to 'F'
        let current_position = (6, 2);
        assert_eq!(is_north_walkable(&matrix, current_position), true);
    }

    #[test]
    fn it_can_check_if_south_is_walkable() {
        let matrix = vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'F', '-', '7', 'F'],
            vec!['L', '|', '7', '|', 'L'],
            vec!['-', 'L', '-', 'J', 'F'],
            vec!['L', '|', '-', 'J', 'J'],
            vec!['L', '|', '-', 'J', '7'],
            vec!['L', '|', '-', 'J', 'J'],
        ];

        // Outside of matrix
        let current_position = (6, 1);
        assert_eq!(is_south_walkable(&matrix, current_position), false);

        // Cannot walk south from 'L'
        let current_position = (3, 1);
        assert_eq!(is_south_walkable(&matrix, current_position), false);

        // Cannot walk south from 'J'
        let current_position = (3, 3);
        assert_eq!(is_south_walkable(&matrix, current_position), false);

        // Cannot walk south from '-'
        let current_position = (3, 2);
        assert_eq!(is_south_walkable(&matrix, current_position), false);

        // Can walk south from 'F' to '|'
        let current_position = (1, 1);
        assert_eq!(is_south_walkable(&matrix, current_position), true);

        // Can walk south from 'F' to 'J'
        let current_position = (3, 4);
        assert_eq!(is_south_walkable(&matrix, current_position), true);

        // Can walk south from 'F' to 'L'
        let current_position = (1, 4);
        assert_eq!(is_south_walkable(&matrix, current_position), true);

        // Can walk south from '|' to 'L'
        let current_position = (2, 1);
        assert_eq!(is_south_walkable(&matrix, current_position), true);

        // Can walk south from '|' to 'J'
        let current_position = (2, 3);
        assert_eq!(is_south_walkable(&matrix, current_position), true);

        // Can walk south from '|' to '|'
        let current_position = (4, 1);
        assert_eq!(is_south_walkable(&matrix, current_position), true);

        // Can walk south from '7' to 'L'
        let current_position = (1, 0);
        assert_eq!(is_south_walkable(&matrix, current_position), true);

        // Can walk south from '7' to 'J'
        let current_position = (5, 4);
        assert_eq!(is_south_walkable(&matrix, current_position), true);

        // Can walk south from '7' to '|'
        let current_position = (1, 3);
        assert_eq!(is_south_walkable(&matrix, current_position), true);
    }

    #[test]
    fn it_can_check_if_east_is_walkable() {
        let matrix = vec![
            vec!['-', 'L', '-', 'F', '7'],
            vec!['-', '|', '-', 'F', 'J'],
            vec!['F', '-', '-', '7', 'J'],
            vec!['F', '-', 'J', 'L', 'J'],
            vec!['L', '7', 'J', 'L', 'J'],
        ];

        // Outside of matrix
        let current_position = (1, 4);
        assert_eq!(is_east_walkable(&matrix, current_position), false);

        // Cannot walk east from '7'
        let current_position = (2, 3);
        assert_eq!(is_east_walkable(&matrix, current_position), false);

        // Cannot walk east from 'J'
        let current_position = (3, 2);
        assert_eq!(is_east_walkable(&matrix, current_position), false);

        // Cannot walk east from '|'
        let current_position = (1, 1);
        assert_eq!(is_east_walkable(&matrix, current_position), false);

        // Can walk east from 'F' to '-'
        let current_position = (2, 1);
        assert_eq!(is_east_walkable(&matrix, current_position), true);

        // Can walk east from 'F' to '7'
        let current_position = (0, 3);
        assert_eq!(is_east_walkable(&matrix, current_position), true);

        // Can walk east from 'F' to 'J'
        let current_position = (1, 3);
        assert_eq!(is_east_walkable(&matrix, current_position), true);

        // Can walk east from 'L' to '-'
        let current_position = (0, 1);
        assert_eq!(is_east_walkable(&matrix, current_position), true);

        // Can walk east from 'L' to '7'
        let current_position = (4, 0);
        assert_eq!(is_east_walkable(&matrix, current_position), true);

        // Can walk east from 'L' to 'J'
        let current_position = (3, 3);
        assert_eq!(is_east_walkable(&matrix, current_position), true);

        // Can walk east from '-' to '7'
        let current_position = (2, 2);
        assert_eq!(is_east_walkable(&matrix, current_position), true);

        // Can walk east from '-' to 'J'
        let current_position = (3, 1);
        assert_eq!(is_east_walkable(&matrix, current_position), true);

        // Can walk east from '-' to '-'
        let current_position = (2, 1);
        assert_eq!(is_east_walkable(&matrix, current_position), true);
    }

    #[test]
    fn it_can_check_if_west_is_walkable() {
        let matrix = vec![
            vec!['-', 'L', '-', 'F', '7'],
            vec!['-', '|', '-', 'F', 'J'],
            vec!['F', '-', '-', '7', 'J'],
            vec!['F', '-', 'J', 'L', 'J'],
            vec!['L', '7', 'J', 'L', 'J'],
        ];

        // Outside of matrix
        let current_position = (1, 0);
        assert_eq!(is_west_walkable(&matrix, current_position), false);

        // Cannot walk west from 'L'
        let current_position = (0, 1);
        assert_eq!(is_west_walkable(&matrix, current_position), false);

        // Cannot walk west from 'F'
        let current_position = (0, 3);
        assert_eq!(is_west_walkable(&matrix, current_position), false);

        // Cannot walk west from '|'
        let current_position = (1, 1);
        assert_eq!(is_west_walkable(&matrix, current_position), false);

        let can_walk_from_position = [
            (2, 3), /* 7 to - */
            (4, 1), /* 7 to L*/
            (0, 4), /* 7 to F */
            (3, 1), /* J to - */
            (4, 4), /* J to L */
            (1, 4), /* J to F */
            (0, 2), /* - to L */
            (2, 1), /* - to F */
            (2, 2), /* - to - */
        ];
        for position in can_walk_from_position.iter() {
            assert_eq!(is_west_walkable(&matrix, *position), true);
        }
    }
}
