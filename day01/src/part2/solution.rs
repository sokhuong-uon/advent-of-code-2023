use utils::get_file_content;

pub fn main() -> String {
    format!("{}", solution(&get_file_content("/day01/src/in.txt")))
}

fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first_digit, last_digit) = fine_first_and_last_digit(line);
            first_digit * 10 + last_digit
        })
        .sum()
}

fn fine_first_and_last_digit(line: &str) -> (u32, u32) {
    let dictionary = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut position_of_first_digit = usize::MAX;
    let mut position_of_last_digit = 0;

    for &(digit_pattern, value) in &dictionary {
        if let Some(position) = line.find(digit_pattern) {
            if position < position_of_first_digit {
                position_of_first_digit = position;
                first_digit = value;
            }
        }
        if let Some(position) = line.rfind(digit_pattern) {
            if position >= position_of_last_digit {
                position_of_last_digit = position;
                last_digit = value;
            }
        }
    }

    (first_digit, last_digit)
}
