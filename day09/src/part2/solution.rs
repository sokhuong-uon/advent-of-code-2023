use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day09/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

fn extract_sequence(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn find_next_value(
    sequence: &Vec<i64>,
    current_index: &mut usize,
    current_values: &mut Vec<i64>,
    last_values: &mut Vec<i64>,
    length_of_previous_values: usize,
) {
    let cloned_first_values = current_values.clone();

    let mut n = sequence[*current_index];
    current_values.push(n);
    for v in cloned_first_values {
        let diff = v - n;
        n = diff;
        current_values.push(diff);

        current_values.remove(0);
    }
    last_values.push(n);

    if current_values[current_values.len() - 1] == 0
        && length_of_previous_values == current_values.len()
    {
        return;
    } else {
        let mut index = *current_index - 1;
        if index > 0 {
            return find_next_value(
                sequence,
                &mut index,
                current_values,
                last_values,
                current_values.len(),
            );
        }
    }
}

fn solution(input: &str) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let mut sequence = extract_sequence(line);
        sequence.reverse();

        let mut current_index = sequence.len() - 1;
        let mut last_values = vec![sequence[current_index]];
        let mut current_values = vec![sequence[current_index]];
        let length_of_previous_values = current_values.len();

        current_index -= 1;
        find_next_value(
            &sequence,
            &mut current_index,
            &mut current_values,
            &mut last_values,
            length_of_previous_values,
        );

        total += last_values.iter().sum::<i64>();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_extract_sequence() {
        assert_eq!(extract_sequence("0 3 6 9 12 15"), vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn it_can_find_next_value() {
        let mut sequence = vec![9, 12, 15];
        sequence.reverse();

        let mut current_index = sequence.len() - 1;
        let mut last_values = vec![sequence[current_index]];
        let mut first_values = vec![sequence[current_index]];
        let length_of_previous_values = first_values.len();

        current_index -= 1;
        find_next_value(
            &sequence,
            &mut current_index,
            &mut first_values,
            &mut last_values,
            length_of_previous_values,
        );
        assert_eq!(last_values, vec![9, -3, 0]);
    }
}
