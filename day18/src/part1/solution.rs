use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

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

type Position = (i32, i32);
type Map = HashMap<Position, char>;

fn extract_data_from_line(line: &str) -> (Direction, u32, String) {
    let mut data = line.split(" ");

    let direction = match data.next().unwrap().chars().next().unwrap() {
        'R' => Direction::East,
        'L' => Direction::West,
        'U' => Direction::North,
        'D' => Direction::South,
        d => panic!("Unknown direction {}", d),
    };

    let blocks = data
        .next()
        .unwrap()
        .parse::<u32>()
        // .inspect(|v| {
        //     println!("v: {:?}", v);
        // })
        .unwrap();

    let color = data.next().unwrap();
    let color = &color[1..color.len() - 1];

    (direction, blocks, color.to_string())
}

fn dig(
    map: &mut Map,
    line: &str,
    current_x: &mut i32,
    current_y: &mut i32,
    max_x: &mut i32,
    max_y: &mut i32,
    least_x: &mut i32,
    least_y: &mut i32,
) {
    let (direction, blocks, _) = extract_data_from_line(line);

    for _ in 1..=blocks {
        match direction {
            Direction::East => *current_x += 1,
            Direction::West => *current_x -= 1,
            Direction::North => *current_y -= 1,
            Direction::South => *current_y += 1,
        }

        map.insert((*current_y, *current_x), '#');

        if current_x > max_x {
            *max_x = *current_x;
        }
        if current_x < least_x {
            *least_x = *current_x;
        }

        if current_y > max_y {
            *max_y = *current_y;
        }
        if current_y < least_y {
            *least_y = *current_y;
        }
    }
}

fn is_north_facing(map: &Map, position: &Position) -> bool {
    let (y, x) = position;

    if map.get(&(y - 1, *x)).is_some() {
        return true;
    }

    false
}

fn count_block_inside(map: &Map, max_x: i32, max_y: i32, min_x: i32, min_y: i32) -> u32 {
    let mut is_inside = false;
    let mut total = 0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.get(&(y, x)).is_some() {
                if is_north_facing(map, &(y, x)) {
                    is_inside = !is_inside;
                }
            } else if is_inside {
                total += 1;
            }
        }
    }

    total
}

fn solution(input: &str) -> u32 {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;

    let mut current_x = 0;
    let mut current_y = 0;

    let mut map: Map = HashMap::new();
    map.insert((0, 0), '#');

    for line in input.lines() {
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
    map.len() as u32 + count_block_inside(&map, max_x, max_y, min_x, min_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_extract_data_from_line() {
        let line = "R 6 (#70c710)";
        assert_eq!(
            (Direction::East, 6, "#70c710".to_string()),
            extract_data_from_line(line)
        )
    }

    #[test]
    fn it_can_dig() {
        let mut map: Map = HashMap::new();
        map.insert((0, 0), '#');

        let mut max_x = 0;
        let mut max_y = 0;
        let mut least_x = 0;
        let mut least_y = 0;

        let mut current_x = 0;
        let mut current_y = 0;

        let line = "R 6 (#70c710)";
        dig(
            &mut map,
            line,
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut least_x,
            &mut least_y,
        );

        assert_eq!(map.len(), 7);
        for x in 0..=6 {
            assert_eq!(map.get(&(0, x)), Some(&'#'));
        }
        assert_eq!(map.get(&(0, 7)), None);

        let line = "D 5 (#0dc571)";
        dig(
            &mut map,
            line,
            &mut current_x,
            &mut current_y,
            &mut max_x,
            &mut max_y,
            &mut least_x,
            &mut least_y,
        );

        assert_eq!(map.len(), 12);
        for y in 0..=5 {
            assert_eq!(map.get(&(y, 6)), Some(&'#'));
        }
        assert_eq!(map.get(&(6, 6)), None);
    }

    #[test]
    fn it_can_check_if_block_is_north_facing() {
        let mut map: Map = HashMap::new();
        map.insert((0, 0), '#');
        map.insert((0, 1), '#');
        map.insert((1, 0), '#');
        // ..
        map.insert((3, 4), '#');

        assert_eq!(is_north_facing(&map, &(0, 0)), false);
        assert_eq!(is_north_facing(&map, &(0, 1)), false);
        assert_eq!(is_north_facing(&map, &(1, 0)), true);
        assert_eq!(is_north_facing(&map, &(3, 4)), false);
    }

    #[test]
    fn it_can_count_block_inside() {
        let mut map: Map = HashMap::new();
        // #######
        // #.....#
        // ###...#
        // ..#...#
        // ..#####

        for x in 0..=6 {
            map.insert((0, x), '#');
        }
        map.insert((1, 0), '#');
        map.insert((1, 6), '#');

        for x in 0..=6 {
            if x < 3 || x > 5 {
                map.insert((2, x), '#');
            }
        }

        map.insert((3, 2), '#');
        map.insert((3, 6), '#');

        for x in 2..=6 {
            map.insert((4, x), '#');
        }

        assert_eq!(map.len(), 20);

        assert_eq!(count_block_inside(&map, 6, 4, 0, 0), 11);

        // #######
        // #.....#
        // ###...#
        // ..#...#
        // ..#...#
        // ###.###
        // #...#..
        // ##..###
        // .#....#
        // .######
        for x in 3..=5 {
            map.remove(&(4, x));
        }
        for x in 0..=6 {
            if x == 3 {
                continue;
            }
            map.insert((5, x), '#');
        }
        map.insert((6, 0), '#');
        map.insert((6, 4), '#');
        for x in 0..=6 {
            if x == 2 || x == 3 {
                continue;
            }
            map.insert((7, x), '#');
        }
        map.insert((8, 1), '#');
        map.insert((8, 6), '#');
        for x in 1..=6 {
            map.insert((9, x), '#');
        }
        assert_eq!(map.len(), 38);
        assert_eq!(count_block_inside(&map, 6, 9, 0, 0), 24);
    }
}
