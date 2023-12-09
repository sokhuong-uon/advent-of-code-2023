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
    first_values: &mut Vec<i64>,
    last_values: &mut Vec<i64>,
) {
    let cloned_first_values = first_values.clone();

    let mut n = sequence[*current_index];
    first_values.push(n);
    for v in cloned_first_values {
        let diff = v - n;
        n = diff;
        first_values.push(diff);

        first_values.remove(0);
    }
    last_values.push(n);

    if first_values[first_values.len() - 1] == 0 {
        return;
    } else {
        *current_index -= 1;
        return find_next_value(sequence, current_index, first_values, last_values);
    }
}

fn solution(input: &str) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let sequence = extract_sequence(line);

        let mut current_index = sequence.len() - 1;
        let mut last_values = vec![sequence[current_index]];
        let mut first_values = vec![sequence[current_index]];

        current_index -= 1;
        find_next_value(
            &sequence,
            &mut current_index,
            &mut first_values,
            &mut last_values,
        );

        let mut index = current_index - 1;
        if index > 0 {
            find_next_value(&sequence, &mut index, &mut first_values, &mut last_values);
        }

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
}
