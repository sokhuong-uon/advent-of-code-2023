use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./src/bin/in.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", p1(&contents));
}

fn convert_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_adjacent_symbols(
    matrix: &Vec<Vec<char>>,
    row_index: usize,
    column_index: usize,
) -> Vec<char> {
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

    let mut symbols: Vec<char> = vec![];

    for adjacent in adjacents {
        if adjacent.0 >= 0
            && adjacent.1 >= 0
            && adjacent.0 < matrix.len() as isize
            && adjacent.1 < matrix[0].len() as isize
        {
            let char = matrix[adjacent.0 as usize][adjacent.1 as usize];
            if char != '.' && !char.is_digit(10) {
                symbols.push(char);
            }
        }
    }

    symbols
}

fn is_adjacent_with_any_symbol(adjacent_symbols: &Vec<char>) -> bool {
    adjacent_symbols.len() > 0
}

fn process_row(matrix: &Vec<Vec<char>>, row: &Vec<char>, row_index: usize) -> u64 {
    let mut part_string = String::new();
    let mut is_adjacent_with_symbol = false;

    let mut sum = 0;

    for (column_index, char) in row.iter().enumerate() {
        if char.is_digit(10) {
            let adjacent_symbols = get_adjacent_symbols(matrix, row_index, column_index);
            if is_adjacent_with_any_symbol(&adjacent_symbols) {
                is_adjacent_with_symbol = true;
            }

            part_string.push(*char);
        } else {
            if is_adjacent_with_symbol && part_string.len() > 0 {
                sum += part_string.parse::<u64>().unwrap();
                is_adjacent_with_symbol = false;
            }
            part_string = String::new();
        }
        println!(
            "{} {} '{}' ({}): {} => {}",
            row_index, column_index, char, part_string, is_adjacent_with_symbol, sum
        );
    }
    // if the last char is numeric
    if is_adjacent_with_symbol && part_string.len() > 0 {
        sum += part_string.parse::<u64>().unwrap();
    }

    sum
}

fn process_matrix(matrix: &Vec<Vec<char>>) -> u64 {
    let mut sum = 0;

    for (row_index, row) in matrix.iter().enumerate() {
        // if row_index > 3 {
        //     break;
        // }

        sum += process_row(matrix, row, row_index);
        println!("sum: {}", sum);
    }

    sum
}

fn p1(input: &str) -> u64 {
    let matrix = convert_input_to_matrix(input);
    let total = process_matrix(&matrix);

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = include_str!("in.txt");
        // let matrix = convert_input_to_matrix(input);
        assert_eq!(p1(input), 4361);
    }
}
