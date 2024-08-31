use utils::get_file_content;

pub fn main() -> String {
    format!("{}", solution(&get_file_content("/day01/src/in.txt")))
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
