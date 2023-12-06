use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day01/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    format!("{}", solution(&contents))
}

fn map_number(str: &str) -> &str {
    match str {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => str,
    }
}

fn solution(input: &str) -> i64 {
    let mut sum = 0;

    let matches = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let mut digit_pair = ("", "");

        let mut left_most_index = line.len();
        let mut right_most_index: isize = -1;

        for (index, number) in matches.iter().enumerate() {
            let left_option = line.find(number);
            if left_option.is_some() {
                let i = left_option.unwrap();
                if i < left_most_index {
                    left_most_index = i;
                    digit_pair.0 = matches[index];
                }
            }

            let right_option = line.rfind(number);
            if right_option.is_some() {
                let i = right_option.unwrap();
                if i as isize > right_most_index {
                    right_most_index = i as isize;
                    digit_pair.1 = matches[index];
                }
            }
        }

        let numeric = format!("{}{}", map_number(digit_pair.0), map_number(digit_pair.1));

        sum += numeric.parse::<i64>().unwrap();
    }

    sum
}
