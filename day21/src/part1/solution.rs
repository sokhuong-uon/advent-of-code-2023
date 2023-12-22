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
type Map = BTreeMap<Position, u8>;
type Position = (usize, usize);
type Matrix = Vec<Vec<char>>;

fn contruct_matrix(input: &str) -> (Matrix, Position) {
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

fn is_able_to_go(matrix: &Matrix, current_position: &Position, direction: &Direction) -> bool {
    let (y, x) = current_position;
    match direction {
        Direction::North => y > &0 && matrix[y - 1][*x] != '#',
        Direction::South => y < &(matrix.len() - 1) && matrix[y + 1][*x] != '#',
        Direction::East => x < &(matrix[0].len() - 1) && matrix[*y][x + 1] != '#',
        Direction::West => x > &0 && matrix[*y][x - 1] != '#',
    }
}

fn visit(map: &mut Map, position: &Position) {
    if let Some(v) = map.get(position) {
        map.insert(*position, v + 1);
    } else {
        map.insert(*position, 1);
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
    let (matrix, start_position) = contruct_matrix(input);
    map.insert(start_position, 1);
    for _ in 0..64 {
        visit_neighbour(&matrix, &mut map);
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
        let (matrix, start_position) = contruct_matrix(input);
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
        let expected_results = vec![false, true, true, false];
        for (direction, expected_result) in directions.iter().zip(expected_results.iter()) {
            assert_eq!(
                is_able_to_go(&matrix, &current_position, &direction),
                *expected_result
            );
        }

        let current_position = (10, 0);
        let expected_results = vec![true, false, true, false];
        for (direction, expected_result) in directions.iter().zip(expected_results.iter()) {
            assert_eq!(
                is_able_to_go(&matrix, &current_position, &direction),
                *expected_result
            );
        }

        let current_position = (10, 10);
        let expected_results = vec![true, false, false, true];
        for (direction, expected_result) in directions.iter().zip(expected_results.iter()) {
            assert_eq!(
                is_able_to_go(&matrix, &current_position, &direction),
                *expected_result
            );
        }

        let current_position = (10, 1);
        let expected_results = vec![false, false, true, true];
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
}
