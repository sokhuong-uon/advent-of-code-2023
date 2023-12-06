use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day04/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    format!("Total point: {}", solution(&contents))
}

fn get_number_group(numeric_line: &str) -> (HashMap<u32, bool>, Vec<u32>) {
    let groups = numeric_line
        .split("|")
        .map(|group| group.trim())
        .collect::<Vec<&str>>();

    let mut winning_number = HashMap::new();
    groups[0]
        .split(" ")
        .filter_map(|numeric| numeric.parse::<u32>().ok())
        .for_each(|number| {
            winning_number.insert(number, true);
        });

    let number_we_have = groups[1]
        .split(" ")
        .filter_map(|numeric| numeric.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    (winning_number, number_we_have)
}

fn card_point(card: &str) -> u32 {
    let mut points = 0;

    let numeric_line = card.split(":").collect::<Vec<&str>>()[1];
    let number_group = get_number_group(numeric_line);

    number_group.1.iter().for_each(|number| {
        if number_group.0.contains_key(number) {
            points = match points {
                0 => 1,
                _ => points * 2,
            }
        }
    });

    points
}

fn solution(input: &str) -> u32 {
    let mut total = 0;

    for card in input.lines() {
        total += card_point(card);
    }

    total
}
