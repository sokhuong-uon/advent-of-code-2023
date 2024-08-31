use std::fs;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let contents = fs::read_to_string(format!("{}/day01/src/in.txt", dir.display())).unwrap();
    format!("{}", solution(&contents))
}

fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            let first = digits.first().unwrap_or(&'0');
            let last = digits.last().unwrap_or(first);
            format!("{}{}", first, last).parse::<u32>().unwrap_or(0)
        })
        .sum()
}
