use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicI64, Ordering};

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day18/src/in.txt", dir.display())).unwrap();
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

type Position = (i64, i64);
type ColumnRange = (i64, i64);
type Map = HashMap<i64, Vec<ColumnRange>>;

fn extract_data_from_line(line: &str) -> (Direction, u32) {
    let mut color = line.split(" ").nth(2).unwrap();
    let len = color.len();
    color = &color[2..len - 1];
    let len = color.len();

    let direction = match &color[len - 1..] {
        "0" => Direction::East,
        "2" => Direction::West,
        "3" => Direction::North,
        "1" => Direction::South,
        _ => panic!("Unknown direction"),
    };
    let distance = u32::from_str_radix(&color[..len - 1], 16).unwrap();
    (direction, distance)
}

fn dig(
    map: &mut Map,
    line: &str,
    current_x: &mut i64,
    current_y: &mut i64,
    max_x: &mut i64,
    max_y: &mut i64,
    min_x: &mut i64,
    min_y: &mut i64,
) {
    let (direction, blocks) = extract_data_from_line(line);
    match direction {
        Direction::East => {
            let start = *current_x + 1;
            let end = start + blocks as i64;
            if let Some(column) = map.get_mut(current_y) {
                column.push((start, end));
            } else {
                map.insert(*current_y, vec![(start, end)]);
            }
            *current_x += blocks as i64;
            if current_x > max_x {
                *max_x = *current_x;
            }
        }
        Direction::West => {
            let start = *current_x - blocks as i64;
            let end = *current_x;
            if let Some(column) = map.get_mut(current_y) {
                column.push((start, end));
            } else {
                map.insert(*current_y, vec![(start, end)]);
            }
            *current_x -= blocks as i64;
            if current_x < min_x {
                *min_x = *current_x;
            }
        }
        Direction::North => {
            let start = *current_x;
            let end = *current_x + 1;
            for _ in 1..=blocks {
                *current_y -= 1;
                if let Some(column) = map.get_mut(current_y) {
                    column.push((start, end));
                } else {
                    map.insert(*current_y, vec![(start, end)]);
                }
            }
            if current_y < min_y {
                *min_y = *current_y;
            }
        }
        Direction::South => {
            let start = *current_x;
            let end = *current_x + 1;
            for _ in 1..=blocks {
                *current_y += 1;
                if let Some(column) = map.get_mut(current_y) {
                    column.push((start, end));
                } else {
                    map.insert(*current_y, vec![(start, end)]);
                }
            }
            if current_y > max_y {
                *max_y = *current_y;
            }
        }
    }
}

fn is_north_facing(map: &Map, position: &Position) -> bool {
    let (y, x) = position;

    if let Some(north_row) = map.get(&(y - 1)) {
        for range in north_row {
            if range.0 <= *x && *x < range.1 {
                return true;
            }
        }
    }

    false
}

fn count_block_inside(map: &mut Map, min_y: i64, max_y: i64) -> u64 {
    println!("Sorting map...");
    map.par_iter_mut().for_each(|(_, block_group)| {
        block_group.sort();
    });

    let total = AtomicI64::new(0);

    println!("Counting block inside...");
    (min_y..=max_y).into_par_iter().for_each(|y| {
        let mut is_inside = false;
        let mut previous_x: Option<i64> = None;

        let block_group = map.get(&y).unwrap();

        for block in block_group {
            if let Some(value) = previous_x {
                if is_inside {
                    total.fetch_add(block.0 - value, Ordering::Relaxed);
                }
            }

            for x in block.0..block.1 {
                if is_north_facing(map, &(y, x)) {
                    is_inside = !is_inside;
                }
            }
            previous_x = Some(block.1);
        }
    });

    total.load(Ordering::Relaxed) as u64
}

fn count_edges(map: &Map) -> u64 {
    println!("Counting edges...");
    let mut total = 0;

    for (_, block_group) in map {
        for (start, end) in block_group {
            total += end - start;
        }
    }

    total as u64
}

fn solution(input: &str) -> u64 {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;

    let mut current_x = 0;
    let mut current_y = 0;

    let mut map: Map = HashMap::new();

    for (instruction_number, line) in input.lines().enumerate() {
        println!("Digging {} ...", instruction_number);
        dig(
            &mut map,
            line,
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut min_x,
            &mut min_y,
        );
    }
    let edges = count_edges(&map);

    edges + count_block_inside(&mut map, min_y, max_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_extract_data_from_line() {
        let line = "R 6 (#70c710)";
        assert_eq!((Direction::East, 461937), extract_data_from_line(line))
    }

    #[test]
    fn it_can_dig_right() {
        let mut map: Map = HashMap::new();
        map.insert(0, vec![(0, 1)]);

        let mut current_x = 0;
        let mut current_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut min_x = 0;
        let mut min_y = 0;

        dig(
            &mut map,
            "R 6 (#000030)",
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut min_x,
            &mut min_y,
        );
        assert_eq!(1, map.len());
        let column = map.get(&0).unwrap();
        assert_eq!(2, column.len());
        assert_eq!(column, &vec![(0, 1), (1, 4)]);

        dig(
            &mut map,
            "R 6 (#000130)",
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut min_x,
            &mut min_y,
        );
        assert_eq!(1, map.len());
        let column = map.get(&0).unwrap();
        assert_eq!(3, column.len());
        assert_eq!(column, &vec![(0, 1), (1, 4), (4, 23)]);
    }

    #[test]
    fn it_can_dig_left() {
        let mut map: Map = HashMap::new();
        map.insert(-1, vec![(-2, 3)]);

        let mut current_x = -2;
        let mut current_y = -1;

        let mut min_x = -2;
        let mut max_x = 3;

        let mut min_y = -1;
        let mut max_y = -1;

        dig(
            &mut map,
            "L 6 (#000012)",
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut min_x,
            &mut min_y,
        );

        assert_eq!(map.len(), 1);
        let column = map.get(&-1).unwrap();
        assert_eq!(column.len(), 2);
        assert_eq!(column, &vec![(-2, 3), (-3, -2)]);
        assert_eq!(current_x, -3);
        assert_eq!(current_y, -1);
        assert_eq!(min_x, -3);
        assert_eq!(max_x, 3);
        assert_eq!(min_y, -1);
        assert_eq!(max_y, -1);

        dig(
            &mut map,
            "L 6 (#000212)",
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut min_x,
            &mut min_y,
        );
        assert_eq!(map.len(), 1);
        let column = map.get(&-1).unwrap();
        assert_eq!(column.len(), 3);
        assert_eq!(column, &vec![(-2, 3), (-3, -2), (-36, -3)]);

        assert_eq!(current_x, -36);
        assert_eq!(current_y, -1);
        assert_eq!(min_x, -36);
        assert_eq!(max_x, 3);
        assert_eq!(min_y, -1);
        assert_eq!(max_y, -1);
    }

    #[test]
    fn it_can_dig_up() {
        let mut map: Map = HashMap::new();
        map.insert(-1, vec![(-2, 3)]);

        let mut current_x = -2;
        let mut current_y = -1;

        let mut min_x = -2;
        let mut max_x = 3;

        let mut min_y = -1;
        let mut max_y = -1;

        dig(
            &mut map,
            "U 6 (#000013)",
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut min_x,
            &mut min_y,
        );

        assert_eq!(map.len(), 2);
        let row = map.get(&-1).unwrap();
        assert_eq!(row.len(), 1);
        assert_eq!(row, &vec![(-2, 3)]);

        let new_row = map.get(&-2).unwrap();
        assert_eq!(new_row.len(), 1);
        assert_eq!(new_row, &vec![(-2, -1)]);

        assert_eq!(current_x, -2);
        assert_eq!(current_y, -2);
        assert_eq!(min_x, -2);
        assert_eq!(max_x, 3);
        assert_eq!(min_y, -2);
        assert_eq!(max_y, -1);

        dig(
            &mut map,
            "U 6 (#000093)",
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut min_x,
            &mut min_y,
        );
        assert_eq!(map.len(), 11);
        let new_row = map.get(&-3).unwrap();
        assert_eq!(new_row.len(), 1);
        let new_row = map.get(&-11).unwrap();
        assert_eq!(new_row.len(), 1);
        assert_eq!(new_row, &vec![(-2, -1)]);

        assert_eq!(current_x, -2);
        assert_eq!(current_y, -11);
        assert_eq!(min_x, -2);
        assert_eq!(max_x, 3);
        assert_eq!(min_y, -11);
        assert_eq!(max_y, -1);
    }

    #[test]
    fn it_can_dig_down() {
        let mut map: Map = HashMap::new();
        map.insert(-1, vec![(-2, 3)]);

        let mut current_x = -2;
        let mut current_y = -1;

        let mut min_x = -2;
        let mut max_x = 3;

        let mut min_y = -1;
        let mut max_y = -1;

        dig(
            &mut map,
            "D 6 (#000011)",
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut min_x,
            &mut min_y,
        );

        assert_eq!(map.len(), 2);
        let row = map.get(&-1).unwrap();
        assert_eq!(row.len(), 1);
        assert_eq!(row, &vec![(-2, 3)]);

        let new_row = map.get(&0).unwrap();
        assert_eq!(new_row.len(), 1);
        assert_eq!(new_row, &vec![(-2, -1)]);

        assert_eq!(current_x, -2);
        assert_eq!(current_y, 0);
        assert_eq!(min_x, -2);
        assert_eq!(max_x, 3);
        assert_eq!(min_y, -1);
        assert_eq!(max_y, 0);

        dig(
            &mut map,
            "D 6 (#000091)",
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut min_x,
            &mut min_y,
        );
        assert_eq!(map.len(), 11);
        let new_row = map.get(&1).unwrap();
        assert_eq!(new_row.len(), 1);
        let new_row = map.get(&9).unwrap();
        assert_eq!(new_row.len(), 1);
        assert_eq!(new_row, &vec![(-2, -1)]);

        let none_row = map.get(&10);
        assert_eq!(none_row, None);

        assert_eq!(current_x, -2);
        assert_eq!(current_y, 9);
        assert_eq!(min_x, -2);
        assert_eq!(max_x, 3);
        assert_eq!(min_y, -1);
        assert_eq!(max_y, 9);
    }

    #[test]
    fn it_can_check_if_block_is_north_facing() {
        let mut map: Map = HashMap::new();
        map.insert(0, vec![(0, 1), (8, 9)]);
        map.insert(1, vec![(0, 1), (8, 9)]);
        map.insert(2, vec![(0, 1), (8, 9)]);
        map.insert(3, vec![(0, 1), (8, 9)]);
        map.insert(4, vec![(0, 1), (8, 9)]);
        map.insert(5, vec![(0, 1), (8, 9)]);
        map.insert(6, vec![(0, 9)]);

        assert_eq!(is_north_facing(&map, &(0, 0)), false);
        assert_eq!(is_north_facing(&map, &(1, 0)), true);
        assert_eq!(is_north_facing(&map, &(2, 0)), true);
        assert_eq!(is_north_facing(&map, &(3, 0)), true);
        assert_eq!(is_north_facing(&map, &(4, 0)), true);
        assert_eq!(is_north_facing(&map, &(5, 0)), true);
        assert_eq!(is_north_facing(&map, &(6, 0)), true);

        assert_eq!(is_north_facing(&map, &(6, 1)), false);
        assert_eq!(is_north_facing(&map, &(6, 2)), false);
        // ...
        assert_eq!(is_north_facing(&map, &(6, 7)), false);
        assert_eq!(is_north_facing(&map, &(6, 8)), true);

        assert_eq!(is_north_facing(&map, &(5, 8)), true);
        assert_eq!(is_north_facing(&map, &(1, 8)), true);
        assert_eq!(is_north_facing(&map, &(0, 8)), false);

        map.insert(-4, vec![(4, 9)]);
        map.insert(-3, vec![(4, 5), (8, 9)]);
        map.insert(-2, vec![(4, 5), (8, 9)]);
        map.insert(-1, vec![(0, 5), (8, 9)]);

        assert_eq!(is_north_facing(&map, &(-4, 4)), false);
        assert_eq!(is_north_facing(&map, &(-4, 5)), false);
        assert_eq!(is_north_facing(&map, &(-4, 8)), false);

        assert_eq!(is_north_facing(&map, &(-3, 4)), true);
        assert_eq!(is_north_facing(&map, &(-3, 5)), true); // not exist
        assert_eq!(is_north_facing(&map, &(-3, 8)), true);

        assert_eq!(is_north_facing(&map, &(-1, 0)), false);
        assert_eq!(is_north_facing(&map, &(-1, 1)), false);
        assert_eq!(is_north_facing(&map, &(-1, 4)), true);
        assert_eq!(is_north_facing(&map, &(-1, 8)), true);
        assert_eq!(is_north_facing(&map, &(-1, 9)), false); // not exist

        assert_eq!(is_north_facing(&map, &(0, 0)), true);
        assert_eq!(is_north_facing(&map, &(0, 8)), true);
    }

    #[test]
    fn it_can_count_block_inside() {
        let mut map: Map = HashMap::new();

        // . . . . # # # # #
        // . . . . # . . . #
        // . . . . # . . . #
        // # # # # # . . . #
        // # . . . . . . . #
        // # . . . . . . . #
        // # . . . . . . . #
        // # . . . . . . . #
        // # . . . . . . . #
        // # . . . . . . . #
        // # # # # # # # # #

        map.insert(-4, vec![(4, 9)]);
        map.insert(-3, vec![(4, 5), (8, 9)]);
        map.insert(-2, vec![(4, 5), (8, 9)]);
        map.insert(-1, vec![(0, 5), (8, 9)]);
        map.insert(0, vec![(0, 1), (8, 9)]);
        map.insert(1, vec![(0, 1), (8, 9)]);
        map.insert(2, vec![(0, 1), (8, 9)]);
        map.insert(3, vec![(0, 1), (8, 9)]);
        map.insert(4, vec![(0, 1), (8, 9)]);
        map.insert(5, vec![(0, 1), (8, 9)]);
        map.insert(6, vec![(0, 9)]);

        assert_eq!(map.len(), 11);

        let min_y = -4;
        let max_y = 6;

        assert_eq!(count_block_inside(&mut map, min_y, max_y), 51);

        // . . . . # # # # #
        // . . . . # . . . #
        // . . . . # . . . #
        // # # # # # . . . #
        // # . . . . . . . #
        // # . . . . . . . #
        // # . . . . . . . #
        // # . # # # # # . #
        // # . # . . . # . #
        // # . # . . . # . #
        // # # # . . . # # #
        let mut map: Map = HashMap::new();

        map.insert(-4, vec![(4, 9)]);
        map.insert(-3, vec![(4, 5), (8, 9)]);
        map.insert(-2, vec![(4, 5), (8, 9)]);
        map.insert(-1, vec![(0, 5), (8, 9)]);
        map.insert(0, vec![(0, 1), (8, 9)]);
        map.insert(1, vec![(0, 1), (8, 9)]);
        map.insert(2, vec![(0, 1), (8, 9)]);
        map.insert(3, vec![(0, 1), (2, 7), (8, 9)]);
        map.insert(4, vec![(0, 1), (2, 3), (6, 7), (8, 9)]);
        map.insert(5, vec![(0, 1), (2, 3), (6, 7), (8, 9)]);
        map.insert(6, vec![(0, 3), (6, 9)]);

        assert_eq!(map.len(), 11);

        let min_y = -4;
        let max_y = 6;

        assert_eq!(count_block_inside(&mut map, min_y, max_y), 36);
    }

    #[test]
    fn it_can_count_edges() {
        // # # # # # # # # #
        // # . . . . . . . #
        // # # # # # . . . #
        // # # # # # . . . #
        // # . . . . . . . #
        // # . . . . . . . #
        // # . . . . . . . #
        // # . # # # # # . #
        // # . # . . . # . #
        // # . # . . . # . #
        // # # # . . . # # #
        let mut map: Map = HashMap::new();

        map.insert(-4, vec![(0, 9)]);
        map.insert(-3, vec![(0, 1), (8, 9)]);
        map.insert(-2, vec![(0, 5), (8, 9)]);
        map.insert(-1, vec![(0, 5), (8, 9)]);
        map.insert(0, vec![(0, 1), (8, 9)]);
        map.insert(1, vec![(0, 1), (8, 9)]);
        map.insert(2, vec![(0, 1), (8, 9)]);
        map.insert(3, vec![(0, 1), (2, 7), (8, 9)]);
        map.insert(4, vec![(0, 1), (2, 3), (6, 7), (8, 9)]);
        map.insert(5, vec![(0, 1), (2, 3), (6, 7), (8, 9)]);
        map.insert(6, vec![(0, 3), (6, 9)]);

        assert_eq!(count_edges(&map), 50);
    }
}
