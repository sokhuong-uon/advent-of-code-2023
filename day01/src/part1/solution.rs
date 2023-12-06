use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day01/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    format!("{}", solution(&contents))
}

fn solution(input: &str) -> i32 {
    let mut pair = (' ', ' ');
    let mut sum = 0;

    for line in input.lines() {
        let chars_vec = line.chars().collect::<Vec<char>>();
        let mut left = 0;
        let mut right = chars_vec.len() - 1;

        while left <= right {
            if pair.0 != ' ' && pair.1 != ' ' {
                break;
            }

            if chars_vec[left].is_digit(10) {
                pair.0 = chars_vec[left];
            } else {
                left += 1;
            }
            if chars_vec[right].is_digit(10) {
                pair.1 = chars_vec[right];
            } else {
                right -= 1;
            }
        }
        let s = format!("{}{}", pair.0, pair.1);
        sum += s.parse::<i32>().unwrap();

        pair = (' ', ' ');
    }
    sum
}
