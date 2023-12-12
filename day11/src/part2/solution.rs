use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day11/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

fn populate_matrix(input: &str, matrix: &mut Matrix) -> (Vec<usize>, Vec<usize>) {
    // Empty space marked with 0
    // Space with at least one galaxy marked with 1
    let mut column_wise_empty_space: Vec<u8> = vec![];
    let mut row_wise_empty_space: Vec<u8> = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        let mut is_row_has_galaxy = false;

        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                is_row_has_galaxy = true;

                if y == 0 {
                    column_wise_empty_space.push(1);
                } else {
                    column_wise_empty_space[x] = 1;
                }
            } else {
                if y == 0 {
                    column_wise_empty_space.push(0);
                }
            }

            row.push(char);
        }

        matrix.push(row);

        if is_row_has_galaxy {
            row_wise_empty_space.push(1);
        } else {
            row_wise_empty_space.push(0);
        }
    }

    let mut empty_column_indices: Vec<usize> = vec![];
    for (mark, column) in column_wise_empty_space.iter().enumerate() {
        if *column == 0 {
            empty_column_indices.push(mark);
        }
    }

    let mut empty_row_indices: Vec<usize> = vec![];
    for (mark, row) in row_wise_empty_space.iter().enumerate() {
        if *row == 0 {
            empty_row_indices.push(mark);
        }
    }

    (empty_row_indices, empty_column_indices)
}

fn calculate_amount_of_empty_space_in_between(
    a: usize,
    b: usize,
    empty_indices: &Vec<usize>,
) -> usize {
    empty_indices
        .iter()
        .filter(|&x| {
            if a < b {
                *x > a && *x < b
            } else {
                *x > b && *x < a
            }
        })
        .count()
}

fn calaculate_distance(
    a: &Position,
    b: &Position,
    empty_row_indices: &Vec<usize>,
    empty_column_indices: &Vec<usize>,
    expansion: usize,
) -> usize {
    let distance_on_y = a.0.abs_diff(b.0);
    let amount_of_empty_rows_in_between =
        calculate_amount_of_empty_space_in_between(a.0, b.0, empty_row_indices);
    let distance_on_y = (distance_on_y - amount_of_empty_rows_in_between)
        + amount_of_empty_rows_in_between * expansion;

    let distance_on_x = a.1.abs_diff(b.1);
    let amount_of_empty_columns_in_between =
        calculate_amount_of_empty_space_in_between(a.1, b.1, empty_column_indices);
    let distance_on_x = (distance_on_x - amount_of_empty_columns_in_between)
        + amount_of_empty_columns_in_between * expansion;

    distance_on_x + distance_on_y
}

type Matrix = Vec<Vec<char>>;
type Position = (usize, usize);

fn solution(input: &str) -> usize {
    let mut total = 0;

    let mut matrix: Matrix = Vec::new();
    let (empty_row_indices, empty_column_indices) = populate_matrix(input, &mut matrix);

    let mut visited_galaxies: Vec<Position> = vec![];
    for (y, row) in matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == '#' {
                for (y1, x1) in visited_galaxies.iter() {
                    let distance = calaculate_distance(
                        &(y, x),
                        &(*y1, *x1),
                        &empty_row_indices,
                        &empty_column_indices,
                        1_000_000,
                    );
                    total += distance;
                }
                visited_galaxies.push((y, x));
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_populate_matrix() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let mut matrix: Matrix = Vec::new();
        populate_matrix(input, &mut matrix);

        assert_eq!(matrix.len(), 10);
        assert_eq!(
            matrix,
            vec![
                vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
                vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.',],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.',],
                vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.',],
                vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.',],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#',],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.',],
                vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.',],
                vec!['#', '.', '.', '.', '#', '.', '.', '.', '.', '.',]
            ]
        );
    }

    #[test]
    fn it_can_get_empty_row_and_column_indices() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let mut matrix: Matrix = Vec::new();
        let (empty_row_indices, empty_column_indices) = populate_matrix(input, &mut matrix);

        assert_eq!(empty_row_indices, vec![3, 7]);
        assert_eq!(empty_column_indices, vec![2, 5, 8])
    }

    #[test]
    fn it_can_calculate_amount_of_empty_space_in_between() {
        let a = 0;
        let b = 4;
        let empty_spaces_indices = vec![2, 5, 8];

        let amount = calculate_amount_of_empty_space_in_between(a, b, &empty_spaces_indices);
        assert_eq!(amount, 1);

        let a = 9;
        let b = 3;
        let empty_spaces_indices = vec![2, 5, 8];

        let amount = calculate_amount_of_empty_space_in_between(a, b, &empty_spaces_indices);
        assert_eq!(amount, 2);
    }

    #[test]
    fn it_can_calculate_distance() {
        let a = (0, 4);
        let b = (2, 0);
        let empty_row_indices = vec![3, 7]; // No empty space in between 0 and 2
        let empty_column_indices = vec![2, 5, 8]; // One empty space in between 0 and 4
        let distance = calaculate_distance(&a, &b, &empty_row_indices, &empty_column_indices, 10);
        assert_eq!(distance, 15);

        let a = (11, 0);
        let b = (11, 8);
        let empty_row_indices = vec![3, 7]; // No empty space in between 11 and 11
        let empty_column_indices = vec![2, 5, 10]; // Two empty spaces in between 0 and 8. (2, 5)
        let distance = calaculate_distance(&a, &b, &empty_row_indices, &empty_column_indices, 20);
        assert_eq!(distance, 46);

        let a = (5, 4);
        let b = (1, 0);
        let empty_row_indices = vec![3, 7]; // One empty space in between 5 and 1. (3)
        let empty_column_indices = vec![2, 5, 8]; // One empty space in between 4 and 0. (2)
        let distance = calaculate_distance(&a, &b, &empty_row_indices, &empty_column_indices, 1);
        assert_eq!(distance, 8);

        let a = (11, 0);
        let b = (11, 5);
        let empty_row_indices = vec![3, 7]; // No empty space in between 11 and 11
        let empty_column_indices = vec![1, 2, 3, 4, 7, 8]; // Four empty spaces in between 0 and 5. (1, 2, 3, 4)
        let distance = calaculate_distance(&a, &b, &empty_row_indices, &empty_column_indices, 3);
        assert_eq!(distance, 13);
    }
}
