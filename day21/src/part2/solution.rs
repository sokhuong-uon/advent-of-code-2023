use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day21/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
type Map = BTreeMap<PositionOnMap, u8>;
type PositionOnMap = (isize, isize);
type Position = (usize, usize);
type Matrix = Vec<Vec<char>>;

fn construct_matrix(input: &str) -> (Matrix, Position) {
    let mut start_position = (0, 0);
    let matrix = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    'S' => {
                        start_position = (y, x);
                        '.'
                    }
                    _ => char,
                })
                .collect::<Vec<char>>()
        })
        .collect::<Matrix>();
    (matrix, start_position)
}

fn is_able_to_go(matrix: &Matrix, current_position: &PositionOnMap, direction: &Direction) -> bool {
    let y_length = matrix.len();
    let x_length = matrix[0].len();

    let (y, x) = current_position;

    let (new_y, new_x) = match direction {
        Direction::North => to_matrix_cordinate(&(y - 1, *x), x_length, y_length),
        Direction::South => to_matrix_cordinate(&(y + 1, *x), x_length, y_length),
        Direction::East => to_matrix_cordinate(&(*y, x + 1), x_length, y_length),
        Direction::West => to_matrix_cordinate(&(*y, x - 1), x_length, y_length),
    };

    matrix[new_y][new_x] != '#'
}

fn to_matrix_cordinate(
    position: &PositionOnMap,
    column_length: usize,
    row_length: usize,
) -> Position {
    let y = (position.0.rem_euclid(row_length as isize)) as usize;
    let x = (position.1.rem_euclid(column_length as isize)) as usize;
    (y, x)
}

fn visit(map: &mut Map, position: &PositionOnMap) {
    if let Some(v) = map.get(position) {
        map.insert(*position, v + 1);
    } else {
        map.insert(*position, 1);
    }
}

fn find_number_of_active_plots_per_edge_incomplete(
    matrix: &Matrix,
    start_position: Position,
) -> usize {
    let mut map = Map::new();
    let (y, x) = start_position;

    map.insert((y as isize, x as isize), 1);
    for _ in 0..matrix.len() {
        visit_neighbour(&matrix, &mut map);
    }
    map.retain(|position, _| {
        position.0 >= 0
            && position.0 < matrix.len() as isize
            && position.1 >= 0
            && position.1 < matrix[0].len() as isize
    });

    map.len()
}

fn find_number_of_active_plots_per_edge_complete(
    matrix: &Matrix,
    start_position: Position,
) -> usize {
    let mut map = Map::new();
    let (y, x) = start_position;

    map.insert((y as isize, x as isize), 1);
    for _ in 0..(matrix.len() * 2) {
        visit_neighbour(&matrix, &mut map);
    }
    map.retain(|position, _| {
        position.0 >= 0
            && position.0 < matrix.len() as isize
            && position.1 >= 0
            && position.1 < matrix[0].len() as isize
    });

    map.len()
}

struct PyramidPartsPlotCount {
    top_left: usize,
    top_right: usize,

    bottom_left: usize,
    bottom_right: usize,

    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

fn count_active_plots_in_pyramid_parts(
    matrix: &Matrix,
    start_position: Position,
) -> PyramidPartsPlotCount {
    let mut map = Map::new();
    let matrix_length = matrix.len();

    let (y, x) = start_position;

    map.insert((y as isize, x as isize), 1);
    for _ in 0..(matrix_length * 2) {
        visit_neighbour(&matrix, &mut map);
    }

    // making top (downward) paramid
    let mut top_pyramid_map = map.clone();
    top_pyramid_map.retain(|position, _| position.0 >= matrix_length as isize * 2);

    // making bottom (upward) paramid
    let mut bottom_pyramid_map = map.clone();
    bottom_pyramid_map.retain(|position, _| position.0 < -(matrix_length as isize));

    // making left (forward) paramid
    let mut left_pyramid_map = map.clone();
    left_pyramid_map.retain(|position, _| position.1 >= matrix_length as isize * 2);

    // making right (backward) paramid
    let mut right_pyramid_map = map.clone();
    right_pyramid_map.retain(|position, _| position.1 < -(matrix_length as isize));

    // making top-left paramid
    let mut top_left_pyramid_map = map.clone();
    top_left_pyramid_map.retain(|position, _| {
        position.1 >= matrix_length as isize && position.0 >= matrix_length as isize
    });

    // making top-right paramid
    let mut top_right_pyramid_map = map.clone();
    top_right_pyramid_map
        .retain(|position, _| position.1 < 0 && position.0 >= matrix_length as isize);

    // making bottom-left paramid
    let mut bottom_left_pyramid_map = map.clone();
    bottom_left_pyramid_map
        .retain(|position, _| position.1 >= matrix_length as isize && position.0 < 0);

    // making bottom-right paramid
    let mut bottom_right_pyramid_map = map.clone();
    bottom_right_pyramid_map.retain(|position, _| position.1 < 0 && position.0 < 0);

    PyramidPartsPlotCount {
        top_left: top_left_pyramid_map.len(),
        top_right: top_right_pyramid_map.len(),

        bottom_left: bottom_left_pyramid_map.len(),
        bottom_right: bottom_right_pyramid_map.len(),

        left: left_pyramid_map.len(),
        top: top_pyramid_map.len(),
        right: right_pyramid_map.len(),
        bottom: bottom_pyramid_map.len(),
    }
}

fn visit_neighbour(matrix: &Matrix, map: &mut Map) {
    let old_map = map.clone();

    old_map.iter().for_each(|(position, _)| {
        if is_able_to_go(matrix, position, &Direction::East) {
            visit(map, &(position.0, position.1 + 1));
        }
        if is_able_to_go(matrix, position, &Direction::West) {
            visit(map, &(position.0, position.1 - 1));
        }
        if is_able_to_go(matrix, position, &Direction::North) {
            visit(map, &(position.0 - 1, position.1));
        }
        if is_able_to_go(matrix, position, &Direction::South) {
            visit(map, &(position.0 + 1, position.1));
        }

        *map.get_mut(position).unwrap() -= 1;
    });

    let new_map = map.clone();
    new_map.iter().for_each(|(position, count)| {
        if count == &0 {
            map.remove(position);
        } else {
            *map.get_mut(position).unwrap() = 1;
        }
    });
}

fn solution(input: &str) -> usize {
    let mut map = Map::new();
    let (matrix, start_position) = construct_matrix(input);
    let (y, x) = start_position;

    map.insert((y as isize, x as isize), 1);
    for i in 1..50 {
        let mut map = Map::new();
        map.insert((y as isize, x as isize), 1);
        for _ in 0..i {
            visit_neighbour(&matrix, &mut map);
        }
        println!("{}", map.len());
    }

    map.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_construct_matrix() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let (matrix, start_position) = construct_matrix(input);
        let expected_matrix = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '#', '#', '#', '.', '#', '.'],
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.'],
            vec!['.', '#', '#', '.', '.', '.', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '#', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.'],
            vec!['.', '#', '#', '.', '#', '.', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '#', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        for (a, b) in matrix.iter().zip(expected_matrix.iter()) {
            assert_eq!(a, b);
        }

        assert_eq!(start_position, (5, 5));
    }

    #[test]
    fn it_can_check_if_able_to_go() {
        let matrix = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '#', '#', '#', '.', '#', '.'],
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.'],
            vec!['.', '#', '#', '.', '.', '.', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '#', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.'],
            vec!['.', '#', '#', '.', '#', '.', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '#', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let current_position = (0, 0);
        let directions = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        let expected_results = vec![true, true, true, true];
        for (direction, expected_result) in directions.iter().zip(expected_results.iter()) {
            assert_eq!(
                is_able_to_go(&matrix, &current_position, &direction),
                *expected_result
            );
        }

        let current_position = (10, 0);
        let expected_results = vec![true, true, true, true];
        for (direction, expected_result) in directions.iter().zip(expected_results.iter()) {
            assert_eq!(
                is_able_to_go(&matrix, &current_position, &direction),
                *expected_result
            );
        }

        let current_position = (10, 10);
        let expected_results = vec![true, true, true, true];
        for (direction, expected_result) in directions.iter().zip(expected_results.iter()) {
            assert_eq!(
                is_able_to_go(&matrix, &current_position, &direction),
                *expected_result
            );
        }

        let current_position = (10, 1);
        let expected_results = vec![false, true, true, true];
        for (direction, expected_result) in directions.iter().zip(expected_results.iter()) {
            assert_eq!(
                is_able_to_go(&matrix, &current_position, &direction),
                *expected_result
            );
        }

        let current_position = (7, 9);
        let expected_results = vec![false, false, true, false];
        for (direction, expected_result) in directions.iter().zip(expected_results.iter()) {
            assert_eq!(
                is_able_to_go(&matrix, &current_position, &direction),
                *expected_result
            );
        }
    }

    #[test]
    fn it_can_visit() {
        let mut map = Map::new();

        let position = (3, 5);

        visit(&mut map, &position);
        assert_eq!(map.get(&position), Some(&1));

        visit(&mut map, &position);
        assert_eq!(map.get(&position), Some(&2));
        assert_eq!(map.len(), 1);

        visit(&mut map, &(-1, 4));
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&(-1, 4)), Some(&1));
    }

    #[test]
    fn it_can_visit_neighbour() {
        let matrix = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '#', '#', '#', '.', '#', '.'],
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.'],
            vec!['.', '#', '#', '.', '.', '.', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '#', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.'],
            vec!['.', '#', '#', '.', '#', '.', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '#', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let mut map = Map::new();
        let position = (5, 5);
        map.insert(position, 1);

        visit_neighbour(&matrix, &mut map); // 1 step
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&(5, 5)), None); // remove itself if no other visit
        assert_eq!(map.get(&(5, 4)), Some(&1)); // can visit west
        assert_eq!(map.get(&(4, 5)), Some(&1)); // can visit north
        assert_eq!(map.get(&(6, 5)), None); // can not visit south
        assert_eq!(map.get(&(5, 6)), None); // can not visit east

        visit_neighbour(&matrix, &mut map); // 2 steps
        assert_eq!(map.len(), 4);
        assert_eq!(map.get(&(5, 6)), None);
        assert_eq!(map.get(&(5, 5)), Some(&1));
        assert_eq!(map.get(&(5, 4)), None);
        assert_eq!(map.get(&(5, 3)), Some(&1));
        assert_eq!(map.get(&(5, 2)), None);

        assert_eq!(map.get(&(6, 2)), None);
        assert_eq!(map.get(&(6, 3)), None);
        assert_eq!(map.get(&(6, 4)), Some(&1));
        assert_eq!(map.get(&(6, 5)), None);

        assert_eq!(map.get(&(4, 3)), None);
        assert_eq!(map.get(&(4, 4)), None);
        assert_eq!(map.get(&(4, 5)), None);
        assert_eq!(map.get(&(4, 6)), None);

        assert_eq!(map.get(&(3, 4)), None);
        assert_eq!(map.get(&(3, 5)), Some(&1));
        assert_eq!(map.get(&(3, 6)), None);

        visit_neighbour(&matrix, &mut map); // 3 steps
        assert_eq!(map.len(), 6);

        visit_neighbour(&matrix, &mut map); // 4 steps
        visit_neighbour(&matrix, &mut map); // 5 steps
        visit_neighbour(&matrix, &mut map); // 6 steps
        assert_eq!(map.len(), 16);
    }

    #[test]
    fn it_can_convert_to_matrix_cordinate() {
        let matrix_row_length = 3;
        let matrix_column_length = 3;

        let position: PositionOnMap = (-1, 0);
        assert_eq!(
            to_matrix_cordinate(&position, matrix_row_length, matrix_column_length),
            (2, 0)
        );

        let position: PositionOnMap = (1, 2);
        assert_eq!(
            to_matrix_cordinate(&position, matrix_row_length, matrix_column_length),
            (1, 2)
        );
    }

    #[test]
    fn it_can_find_number_of_active_plots_per_edge_incomplete() {
        let matrix = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '#', '.', '#', '.'],
            vec!['.', '#', '#', '#', '.', '.', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '#', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.'],
            vec!['.', '#', '#', '.', '#', '.', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '.', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let result = find_number_of_active_plots_per_edge_incomplete(&matrix, (5, 5));
        assert_eq!(result, 44);
    }

    #[test]
    fn it_can_find_number_of_active_plots_per_edge_complete() {
        let matrix = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '#', '.', '#', '.'],
            vec!['.', '#', '#', '#', '.', '.', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '#', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.'],
            vec!['.', '#', '#', '.', '#', '.', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '.', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let result = find_number_of_active_plots_per_edge_complete(&matrix, (5, 5));
        assert_eq!(result, 47);
    }

    #[test]
    fn it_can_count_active_plots_in_pyramid_parts() {
        let matrix = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '#', '.', '#', '.'],
            vec!['.', '#', '#', '#', '.', '.', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '#', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.'],
            vec!['.', '#', '#', '.', '#', '.', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '.', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let pyramid_parts_counts = count_active_plots_in_pyramid_parts(&matrix, (5, 5));
        assert_eq!(pyramid_parts_counts.top, 14);
        assert_eq!(pyramid_parts_counts.bottom, 14);
        assert_eq!(pyramid_parts_counts.left, 16);
        assert_eq!(pyramid_parts_counts.right, 15);

        assert_eq!(pyramid_parts_counts.top_left, 27);
        assert_eq!(pyramid_parts_counts.top_right, 26);

        assert_eq!(pyramid_parts_counts.bottom_left, 26);
        assert_eq!(pyramid_parts_counts.bottom_right, 26);
    }
}
