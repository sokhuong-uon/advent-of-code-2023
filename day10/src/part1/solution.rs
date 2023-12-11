use std::fs::File;
use std::io::Read;

use super::is_walkable::is_walkable;
use super::is_walkable::Direction;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day10/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

pub fn get_vec_of_char_from_line(line: &str) -> (Vec<char>, Option<usize>) {
    let mut column: Option<usize> = None;
    let mut vec = Vec::new();
    for (i, char) in line.chars().enumerate() {
        if char == 'S' {
            column = Some(i);
        }
        vec.push(char);
    }
    (vec, column)
}

pub fn get_north_cordinate(matrix: &Matrix, current_position: Position) -> Option<Position> {
    if is_walkable(matrix, current_position, Direction::North) {
        Some((current_position.0 - 1, current_position.1))
    } else {
        None
    }
}

pub fn get_south_cordinate(matrix: &Matrix, current_position: Position) -> Option<Position> {
    if is_walkable(matrix, current_position, Direction::South) {
        Some((current_position.0 + 1, current_position.1))
    } else {
        None
    }
}

pub fn get_east_cordinate(matrix: &Matrix, current_position: Position) -> Option<Position> {
    if is_walkable(matrix, current_position, Direction::East) {
        Some((current_position.0, current_position.1 + 1))
    } else {
        None
    }
}

pub fn get_west_cordinate(matrix: &Matrix, current_position: Position) -> Option<Position> {
    if is_walkable(matrix, current_position, Direction::West) {
        Some((current_position.0, current_position.1 - 1))
    } else {
        None
    }
}

pub fn populate_matrix(matrix: &mut Matrix, input: &str) -> Position {
    let mut start_position = (0, 0);
    for (i, line) in input.lines().enumerate() {
        let (vec, column) = get_vec_of_char_from_line(line);
        if let Some(column) = column {
            start_position = (i, column);
        }
        matrix.push(vec);
    }
    start_position
}

pub fn find_cordinate_of_pipes_connected_to_start_position(
    matrix: &mut Matrix,
    start_position: Position,
) -> (Position, Position) {
    let possible_chars = vec!['|', '7', 'F', 'J', 'L', '-'];
    let mut cordinates: Vec<Vec<Position>> = vec![];

    possible_chars.iter().for_each(|char| {
        matrix[start_position.0][start_position.1] = char.clone();

        let possible_cordinates = vec![
            get_north_cordinate(matrix, start_position),
            get_south_cordinate(matrix, start_position),
            get_east_cordinate(matrix, start_position),
            get_west_cordinate(matrix, start_position),
        ]
        .iter()
        .filter_map(|cordinate| *cordinate)
        .collect();

        cordinates.push(possible_cordinates);
    });

    let connected_cordinate = cordinates
        .iter()
        .filter(|cordinate| cordinate.len() == 2)
        .collect::<Vec<&Vec<Position>>>();

    (connected_cordinate[0][0], connected_cordinate[0][1])
}

pub fn get_next_position(
    matrix: &Matrix,
    current_position: Position,
    previous_position: Position,
) -> Position {
    let north_cordinate = get_north_cordinate(matrix, current_position);
    let south_cordinate = get_south_cordinate(matrix, current_position);
    let east_cordinate = get_east_cordinate(matrix, current_position);
    let west_cordinate = get_west_cordinate(matrix, current_position);

    let next_cordinate = vec![
        north_cordinate,
        south_cordinate,
        east_cordinate,
        west_cordinate,
    ]
    .iter()
    .filter_map(|cordinate| *cordinate)
    .filter(|cordinate| *cordinate != previous_position)
    .next()
    .unwrap();

    next_cordinate
}

pub type Matrix = Vec<Vec<char>>;
pub type Position = (usize, usize);

fn solution(input: &str) -> i64 {
    let mut total = 0;
    let mut matrix: Matrix = Vec::new();
    let start_position = populate_matrix(&mut matrix, input);
    let (mut cordinate1, mut cordinate2) =
        find_cordinate_of_pipes_connected_to_start_position(&mut matrix, start_position);

    let mut previous_position1 = start_position;
    let mut previous_position2 = start_position;

    while cordinate1 != cordinate2 {
        let next_position1 = get_next_position(&matrix, cordinate1, previous_position1);
        let next_position2 = get_next_position(&matrix, cordinate2, previous_position2);

        previous_position1 = cordinate1;
        previous_position2 = cordinate2;

        cordinate1 = next_position1;
        cordinate2 = next_position2;

        total += 1;
    }
    total += 1;

    total
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_can_get_vec_of_char_from_line() {
        let line = "-L|F7";
        let expected = vec!['-', 'L', '|', 'F', '7'];
        assert_eq!(super::get_vec_of_char_from_line(line).0, expected);
    }

    #[test]
    fn it_can_get_cordinate() {
        let matrix = vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'F', '-', '7', '|'],
            vec!['L', '|', '7', '|', '|'],
            vec!['-', 'L', '-', 'J', '|'],
            vec!['L', '|', '-', 'J', 'F'],
        ];

        let current_position = (1, 1);
        let north_cordinate = super::get_north_cordinate(&matrix, current_position);
        assert_eq!(north_cordinate, None);

        let south_cordinate = super::get_south_cordinate(&matrix, current_position);
        assert_eq!(south_cordinate, Some((2, 1)));

        let east_cordinate = super::get_east_cordinate(&matrix, current_position);
        assert_eq!(east_cordinate, Some((1, 2)));

        let west_cordinate = super::get_west_cordinate(&matrix, current_position);
        assert_eq!(west_cordinate, None);
    }

    #[test]
    fn it_can_populate_matrix() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        let mut matrix: super::Matrix = Vec::new();
        super::populate_matrix(&mut matrix, input);

        assert_eq!(matrix.len(), 5);
        assert_eq!(matrix[0].len(), 5);
        assert_eq!(matrix[1].len(), 5);
        assert_eq!(matrix[2].len(), 5);
        assert_eq!(matrix[3].len(), 5);
        assert_eq!(matrix[4].len(), 5);
    }

    #[test]
    fn it_can_find_the_right_start_position() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        let mut matrix: super::Matrix = Vec::new();
        let start_position = super::populate_matrix(&mut matrix, input);

        assert_eq!(start_position, (1, 1));
        assert_eq!(matrix[1][1], 'S');
    }

    #[test]
    fn it_can_find_cordinate_of_pipes_connected_to_start_position() {
        let mut matrix = vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'S', '-', '7', '|'],
            vec!['L', '|', '7', '|', '|'],
            vec!['-', 'L', '-', 'J', '|'],
            vec!['L', '|', '-', 'J', 'F'],
        ];

        let current_position = (1, 1);

        let (cordinate1, cordinate2) = super::find_cordinate_of_pipes_connected_to_start_position(
            &mut matrix,
            current_position,
        );

        // Order of cordinates does not matter in this case
        assert!(cordinate1 == (1, 2) || cordinate1 == (2, 1));
        assert!(cordinate2 == (1, 2) || cordinate2 == (2, 1));
    }

    #[test]
    fn it_can_get_next_position() {
        let matrix = vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'S', '-', '7', '|'],
            vec!['L', '|', '7', '|', '|'],
            vec!['-', 'L', '-', 'J', '|'],
            vec!['L', '|', '-', 'J', 'F'],
        ];

        let previous_position = (1, 1);
        let current_position = (1, 2);

        let next_position = super::get_next_position(&matrix, current_position, previous_position);
        assert_eq!(next_position, (1, 3));

        let previous_position = (1, 1);
        let current_position = (2, 1);

        let next_position = super::get_next_position(&matrix, current_position, previous_position);
        assert_eq!(next_position, (3, 1));
    }
}
