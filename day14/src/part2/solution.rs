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
                rocks_and_blocks.push(vec![('O', 1)]);
            }
            '#' => {
                rocks_and_blocks.push(vec![('#', 0)]);
            }
            '.' => {
                rocks_and_blocks.push(vec![('O', 0)]);
            }
            _ => {}
        }
    }
    println!("{:?}", rocks_and_blocks);

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

fn populate_matrix(matrix: &mut Vec<Vec<char>>, input: &str) {
    *matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
}

type Rock = (char, usize, usize);

fn get_cube_rock_record(matrix: &Vec<Vec<char>>) -> (Vec<Vec<Rock>>, Vec<Vec<Rock>>) {
    let number_of_row = matrix.len();
    let number_of_column = matrix[0].len();

    let mut column_wise_cube_rocks: Vec<Vec<Rock>> = vec![];
    for _ in 0..number_of_column {
        column_wise_cube_rocks.push(vec![]);
    }

    let mut row_wise_cube_rocks: Vec<Vec<Rock>> = vec![];
    for _ in 0..number_of_row {
        row_wise_cube_rocks.push(vec![]);
    }

    for (y, row) in matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            match char {
                '#' => {
                    column_wise_cube_rocks[x].push(('#', y, y + 1));
                    row_wise_cube_rocks[y].push(('#', x, x + 1));
                }
                _ => {}
            }
        }
    }

    (row_wise_cube_rocks, column_wise_cube_rocks)
}

fn move_north_initial(
    matrix: &Vec<Vec<char>>,
    column_wise_cube_rocks: &Vec<Vec<Rock>>,
) -> Vec<Vec<Rock>> {
    // .....#....
    // ....#...O#
    // ...OO##...
    // .OO#......
    // .....OOO#.
    // .O#...O#.#
    // ....O#....
    // ......OOOO
    // #...O###..
    // #..OO#....

    let mut round_rocks = column_wise_cube_rocks.clone();

    for (y, row) in matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            match char {
                'O' => {
                    if round_rocks[x].len() == 0 {
                        round_rocks[x].push(('O', y, y + 1));
                    } else {
                        let mut new_round_rocks = round_rocks[x].clone();
                        let mut increment = 0;
                        let original_length = round_rocks[x].len();

                        let mut previous_rock_option: Option<&mut Rock> = None;
                        for (index, rock) in round_rocks[x].iter_mut().enumerate() {
                            println!("{:?}", rock);
                            if y < rock.1 {
                                if let Some(previous_rock) = previous_rock_option {
                                    if previous_rock.0 == 'O' {
                                        previous_rock.2 += 1;
                                    } else if previous_rock.0 == '#' {
                                        new_round_rocks.insert(
                                            index - 1 + increment,
                                            ('O', previous_rock.2, previous_rock.2 + 1),
                                        );
                                        increment += 1;
                                    }
                                } else {
                                    if index == 0 {
                                        new_round_rocks.insert(0, ('O', 0, 1));
                                    } else {
                                        new_round_rocks.insert(index - 1 + increment, ('O', 0, 1));
                                    }
                                    increment += 1;
                                }
                            } else {
                                println!("wow;{} {}", index, original_length - 1);
                                if index == original_length - 1 {
                                    println!("aha");
                                    println!("{}", previous_rock_option.is_some());
                                    if let Some(previous_rock) = previous_rock_option {
                                        println!("{} {}", previous_rock.0, previous_rock.2);
                                        if previous_rock.0 == 'O' {
                                            previous_rock.2 += 1;
                                        } else {
                                            new_round_rocks.push((
                                                'O',
                                                previous_rock.2,
                                                previous_rock.2 + 1,
                                            ));
                                        }
                                    }
                                }
                            }

                            previous_rock_option = Some(rock);
                        }

                        round_rocks[x] = new_round_rocks;
                    }
                }
                _ => {}
            }
        }
    }

    round_rocks
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
    fn it_can_populate_matrix() {
        let mut matrix: Vec<Vec<char>> = vec![];

        let input = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

        populate_matrix(&mut matrix, input);
        assert_eq!(
            matrix[0],
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
        );
        assert_eq!(
            matrix[1],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', 'O', '#'],
        );
        assert_eq!(
            matrix[2],
            vec!['.', '.', '.', 'O', 'O', '#', '#', '.', '.', '.'],
        );
        assert_eq!(
            matrix[3],
            vec!['.', 'O', 'O', '#', '.', '.', '.', '.', '.', '.'],
        );
        assert_eq!(
            matrix[4],
            vec!['.', '.', '.', '.', '.', 'O', 'O', 'O', '#', '.'],
        );
        assert_eq!(
            matrix[5],
            vec!['.', 'O', '#', '.', '.', '.', 'O', '#', '.', '#'],
        );
        assert_eq!(
            matrix[6],
            vec!['.', '.', '.', '.', 'O', '#', '.', '.', '.', '.'],
        );

        assert_eq!(
            matrix[7],
            vec!['.', '.', '.', '.', '.', '.', 'O', 'O', 'O', 'O'],
        );
        assert_eq!(
            matrix[8],
            vec!['#', '.', '.', '.', 'O', '#', '#', '#', '.', '.'],
        );
        assert_eq!(
            matrix[9],
            vec!['#', '.', '.', 'O', 'O', '#', '.', '.', '.', '.'],
        );
    }

    #[test]
    fn it_can_get_cube_rock_record() {
        let mut matrix: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '#', '.'],
            vec!['.', '.', 'O', 'O', '#'],
            vec!['.', 'O', '#', '.', '.'],
            vec!['.', 'O', '.', '.', '.'],
            vec!['.', '.', '.', 'O', '#'],
            vec!['.', '.', '.', '.', '.'],
            vec!['#', '.', '.', 'O', '#'],
            vec!['#', '.', 'O', 'O', '#'],
        ];

        let (row_wise_cube_rock_record, column_wise_cube_rock_record) =
            get_cube_rock_record(&matrix);

        assert_eq!(
            row_wise_cube_rock_record,
            vec![
                vec![('#', 4, 5)],
                vec![('#', 3, 4)],
                vec![('#', 4, 5)],
                vec![('#', 2, 3)],
                vec![],
                vec![('#', 4, 5)],
                vec![],
                vec![('#', 0, 1), ('#', 4, 5)],
                vec![('#', 0, 1), ('#', 4, 5)],
            ]
        );

        assert_eq!(
            column_wise_cube_rock_record,
            vec![
                vec![('#', 7, 8), ('#', 8, 9)],
                vec![],
                vec![('#', 3, 4)],
                vec![('#', 1, 2)],
                vec![
                    ('#', 0, 1),
                    ('#', 2, 3),
                    ('#', 5, 6),
                    ('#', 7, 8),
                    ('#', 8, 9)
                ],
            ]
        );
    }

    #[test]
    fn it_can_more_north_initial() {
        let mut matrix: Vec<Vec<char>> = vec![
            vec!['O', 'O', '#'],
            vec!['#', '.', '.'],
            vec!['.', 'O', '#'],
            vec!['O', 'O', '#'],
        ];

        let (_, column_wise_cube_rock_record) = get_cube_rock_record(&matrix);

        let north = move_north_initial(&matrix, &column_wise_cube_rock_record);

        assert_eq!(north[0], vec![('O', 0, 1), ('#', 1, 2), ('#', 2, 3)]);
    }
}
