use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day11/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

fn populate_row_of_matrix(line: &str, matrix: &mut Matrix, column_wise_galaxy: &mut Vec<u8>) {
    let mut row = vec![];
    let mut is_row_has_galaxy = false;

    for (i, char) in line.chars().enumerate() {
        if char == '#' {
            column_wise_galaxy[i] = 1;
            is_row_has_galaxy = true;
        }
        row.push(char);
    }

    if !is_row_has_galaxy {
        matrix.push(row.clone());
    }
    matrix.push(row);
}

fn populate_matrix(input: &str, matrix: &mut Matrix) {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut column_wise_galaxy = vec![0u8; first_line.len()];
    populate_row_of_matrix(first_line, matrix, &mut column_wise_galaxy);

    while let Some(line) = lines.next() {
        populate_row_of_matrix(line, matrix, &mut column_wise_galaxy);
    }

    let mut column_without_galaxy: Vec<usize> = vec![];

    let mut expansion_count = 0;
    for (i, flag) in column_wise_galaxy.iter().enumerate() {
        if *flag == 0 {
            column_without_galaxy.push(i + expansion_count);
            expansion_count += 1;
        }
    }

    for row in matrix.iter_mut() {
        for column in column_without_galaxy.iter() {
            row.insert(*column, '.');
        }
    }
}

fn calaculate_distance(a: &Position, b: &Position) -> usize {
    let distance_on_y = a.0.abs_diff(b.0);
    let distance_on_x = a.1.abs_diff(b.1);
    distance_on_x + distance_on_y
}

type Matrix = Vec<Vec<char>>;
type Position = (usize, usize);

fn solution(input: &str) -> usize {
    let mut total = 0;

    let mut matrix: Matrix = Vec::new();
    populate_matrix(input, &mut matrix);

    let mut visited_galaxies: Vec<Position> = vec![];
    for (y, row) in matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == '#' {
                for (y1, x1) in visited_galaxies.iter() {
                    let distance = calaculate_distance(&(y, x), &(*y1, *x1));
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
    fn it_can_populate_row_of_matrix() {
        let line = "...#......";
        let mut matrix: Matrix = Vec::new();
        let mut column_wise_galaxy = vec![0; line.len()];

        populate_row_of_matrix(line, &mut matrix, &mut column_wise_galaxy);

        assert_eq!(matrix.len(), 1);
        assert_eq!(
            matrix[0],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.']
        );
        assert_eq!(column_wise_galaxy, vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0]);

        let line = "..........";
        let mut matrix: Matrix = Vec::new();
        let mut column_wise_galaxy = vec![0; line.len()];

        populate_row_of_matrix(line, &mut matrix, &mut column_wise_galaxy);

        assert_eq!(matrix.len(), 2);
        assert_eq!(
            matrix[0],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.']
        );
        assert_eq!(
            matrix[1],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.']
        );
        assert_eq!(column_wise_galaxy, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

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

        assert_eq!(matrix.len(), 12);
        assert_eq!(
            matrix,
            vec![
                vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
                vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.',],
                vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#',],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.',],
                vec!['#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.',]
            ]
        );
    }

    #[test]
    fn it_can_calculate_distance() {
        let a = (0, 4);
        let b = (2, 0);
        let distance = calaculate_distance(&a, &b);
        assert_eq!(distance, 6);

        let a = (11, 0);
        let b = (11, 5);
        let distance = calaculate_distance(&a, &b);
        assert_eq!(distance, 5);
    }
}
