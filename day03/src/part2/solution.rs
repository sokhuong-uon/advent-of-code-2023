use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day03/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

fn convert_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn remove_duplicate_adjacent(adjacents: &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut unique_adjacents: Vec<(isize, isize)> = vec![adjacents[0]];
    let mut reference = adjacents[0];

    for i in 1..adjacents.len() {
        if adjacents[i].0 == reference.0
            && (adjacents[i].1 == reference.1 - 1 || adjacents[i].1 == reference.1 + 1)
        {
            reference = adjacents[i];
            continue;
        }
        unique_adjacents.push(adjacents[i]);
        reference = adjacents[i];
    }
    unique_adjacents
}

fn get_adjacent_numeric(
    matrix: &Vec<Vec<char>>,
    row_index: usize,
    column_index: usize,
) -> Vec<(isize, isize)> {
    let top_left = (row_index as isize - 1, column_index as isize - 1);
    let top = (row_index as isize - 1, column_index as isize);
    let top_right = (row_index as isize - 1, column_index as isize + 1);
    let left = (row_index as isize, column_index as isize - 1);
    let right = (row_index as isize, column_index as isize + 1);
    let bottom_left = (row_index as isize + 1, column_index as isize - 1);
    let bottom = (row_index as isize + 1, column_index as isize);
    let bottom_right = (row_index as isize + 1, column_index as isize + 1);

    let adjacents = vec![
        top_left,
        top,
        top_right,
        left,
        right,
        bottom_left,
        bottom,
        bottom_right,
    ];

    let mut symbols: Vec<(isize, isize)> = vec![];

    for adjacent in adjacents {
        if adjacent.0 >= 0
            && adjacent.1 >= 0
            && adjacent.0 < matrix.len() as isize
            && adjacent.1 < matrix[0].len() as isize
        {
            let char = matrix[adjacent.0 as usize][adjacent.1 as usize];
            if char.is_digit(10) {
                symbols.push(adjacent);
            }
        }
    }

    symbols
}

fn is_gear(adjacent_numeric: &Vec<(isize, isize)>) -> bool {
    adjacent_numeric.len() == 2
}

fn compute_gear_ratio(matrix: &Vec<Vec<char>>, adjacent_numeric: &Vec<(isize, isize)>) -> u64 {
    let mut ratio = 1;

    let mut numeric = String::new();

    for adjacent in adjacent_numeric {
        numeric.push(matrix[adjacent.0 as usize][adjacent.1 as usize]);

        // seek to the right
        let mut right = adjacent.1 as usize;
        while right + 1 < matrix[0].len() {
            let char = matrix[adjacent.0 as usize][right + 1];
            if char.is_digit(10) {
                numeric.push(char);
            } else {
                break;
            }
            right += 1;
        }

        // seek to the left
        let mut left = adjacent.1;
        while left - 1 >= 0 {
            let char = matrix[adjacent.0 as usize][left as usize - 1];
            if char.is_digit(10) {
                numeric.insert(0, char)
            } else {
                break;
            }
            left -= 1;
        }

        ratio *= numeric.parse::<u64>().unwrap();

        numeric.clear();
    }

    ratio
}

fn process_row(matrix: &Vec<Vec<char>>, row: &Vec<char>, row_index: usize) -> u64 {
    let mut sum = 0;

    for (column_index, char) in row.iter().enumerate() {
        if char == &'*' {
            let adjacent_numeric = get_adjacent_numeric(matrix, row_index, column_index);
            if adjacent_numeric.len() < 2 {
                continue;
            }

            let adjacent_numeric = remove_duplicate_adjacent(&adjacent_numeric);
            if is_gear(&adjacent_numeric) {
                let ratio = compute_gear_ratio(matrix, &adjacent_numeric);
                sum += ratio;
            }
        }
    }

    sum
}

fn process_matrix(matrix: &Vec<Vec<char>>) -> u64 {
    let mut sum = 0;

    for (row_index, row) in matrix.iter().enumerate() {
        sum += process_row(matrix, row, row_index);
    }

    sum
}

fn solution(input: &str) -> u64 {
    let matrix = convert_input_to_matrix(input);
    let total = process_matrix(&matrix);

    total
}
