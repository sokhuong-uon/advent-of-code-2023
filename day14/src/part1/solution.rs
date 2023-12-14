use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day14/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

fn move_rock_to_the_north(input: &str) -> (Vec<Vec<(char, usize)>>, usize) {
    let mut rocks_and_blocks: Vec<Vec<(char, usize)>> = vec![];

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();

    for char in first_line.chars() {
        match char {
            'O' => {
                rocks_and_blocks.push(vec![('O', 0)]);
            }
            '#' => {
                rocks_and_blocks.push(vec![('#', 0)]);
            }
            '.' => {
                rocks_and_blocks.push(vec![('.', 0)]);
            }
            _ => {}
        }
    }

    let mut index: usize = 1;

    while let Some(line) = lines.next() {
        for (column, char) in line.chars().enumerate() {
            match char {
                'O' => {
                    let length = rocks_and_blocks[column].len();

                    match rocks_and_blocks[column][length - 1].0 {
                        'O' => {
                            rocks_and_blocks[column][length - 1].1 += 1;
                        }
                        '#' => {
                            let last_block = rocks_and_blocks[column][length - 1];
                            rocks_and_blocks[column].push(('O', last_block.1 + 1));
                        }
                        '.' => {
                            rocks_and_blocks[column].push(('O', 0));
                        }
                        _ => {}
                    }
                }
                '#' => {
                    rocks_and_blocks[column].push(('#', index));
                }
                _ => {}
            }
        }
        index += 1;
    }

    (rocks_and_blocks.clone(), index)
}

fn calculate_load(rocks_and_blocks: &Vec<(char, usize)>, pattern_length: usize) -> usize {
    let mut total = 0;
    let mut last_block = 0;
    let mut initial = true;

    for rock_or_block in rocks_and_blocks {
        match rock_or_block.0 {
            'O' => {
                for i in (if initial { 0 } else { last_block + 1 })..(rock_or_block.1 + 1) {
                    total += pattern_length - i as usize;
                }
                initial = false;
            }
            '#' => {
                last_block = rock_or_block.1;
                initial = false;
            }
            '.' => {
                last_block = 0;
                initial = true;
            }
            _ => {}
        }
    }

    total
}

fn solution(input: &str) -> usize {
    let mut total = 0;

    let (rocks_and_blocks, distance_from_north_to_south) = move_rock_to_the_north(input);

    for rocks_and_blocks in rocks_and_blocks {
        total += calculate_load(&rocks_and_blocks, distance_from_north_to_south);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_move_rock_to_the_north() {
        let input = "O.
O.
..
OO
.O";

        let rocks_and_blocks = move_rock_to_the_north(input);
        assert_eq!(
            rocks_and_blocks.0,
            vec![vec![('O', 2)], vec![('.', 0), ('O', 1)],]
        );

        let input = "O....
O.OO#
.....
OO.#O
.O...
O.#..
..O..
.....
#....
#OO..";

        let rocks_and_blocks = move_rock_to_the_north(input);
        assert_eq!(
            rocks_and_blocks.0,
            vec![
                vec![('O', 3), ('#', 8), ('#', 9)],
                vec![('.', 0), ('O', 2)],
                vec![('.', 0), ('O', 0), ('#', 5), ('O', 7)],
                vec![('.', 0), ('O', 0), ('#', 3)],
                vec![('.', 0), ('#', 1), ('O', 2)], //
            ]
        );

        let input = "#.
..
##
..
..
O.
#O
..
##
#.";

        let rocks_and_blocks = move_rock_to_the_north(input);
        assert_eq!(
            rocks_and_blocks.0,
            vec![
                vec![('#', 0), ('#', 2), ('O', 3), ('#', 6), ('#', 8), ('#', 9)],
                vec![('.', 0), ('#', 2), ('O', 3), ('#', 8)],
            ]
        )
    }
}
