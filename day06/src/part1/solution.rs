use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day06/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total number of ways to win: {}", solution(&contents))
}

fn extract_values_from_line(line: &str) -> Vec<u64> {
    line.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .filter_map(|numeric_string| numeric_string.parse::<u64>().ok())
        .collect::<Vec<u64>>()
}

fn find_number_of_way_to_win_a_game(time_available: &u64, distance: &u64) -> u64 {
    let mut hold = 1;

    while hold < *time_available {
        if hold * (time_available - hold) > *distance {
            break;
        }
        hold += 1;
    }

    time_available - hold + 1 - hold
}

fn solution(input: &str) -> u64 {
    let times = extract_values_from_line(input.lines().nth(0).unwrap());
    let distances = extract_values_from_line(input.lines().nth(1).unwrap());

    times
        .iter()
        .enumerate()
        .map(|(i, time)| find_number_of_way_to_win_a_game(time, &distances[i]))
        .fold(1, |acc, current| acc * current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_extract_values_from_line() {
        let time = "Time:      7  15   30";
        let times = extract_values_from_line(time);
        assert_eq!(times, vec![7, 15, 30]);

        let distance = "Distance:  9  40  200";
        let distances = extract_values_from_line(distance);
        assert_eq!(distances, vec![9, 40, 200]);
    }

    #[test]
    fn it_can_find_number_of_way_to_win_a_game() {
        assert_eq!(find_number_of_way_to_win_a_game(&7, &9), 4);
        assert_eq!(find_number_of_way_to_win_a_game(&15, &40), 8);
        assert_eq!(find_number_of_way_to_win_a_game(&30, &200), 9);
    }
}
