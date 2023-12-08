use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day08/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total step required: {}", solution(&contents))
}

fn extract_instruction(lines: &mut std::str::Lines) -> Vec<char> {
    lines.next().unwrap().chars().collect::<Vec<char>>()
}

fn extract_map_item(line: &str) -> (&str, (&str, &str)) {
    let mut line = line.split(" ");
    let node = line.next().unwrap();
    line.next();
    let left = line.next().unwrap();
    let left = &left[1..left.len() - 1];
    let right = line.next().unwrap();
    let right = &right[0..right.len() - 1];

    (node, (left, right))
}

fn advance_instruction_index(instruction: &Vec<char>, current_instruction_index: usize) -> usize {
    let mut current_instruction_index = current_instruction_index;
    current_instruction_index += 1;
    if current_instruction_index >= instruction.len() {
        current_instruction_index = 0;
    }
    current_instruction_index
}

fn solution(input: &str) -> u64 {
    let mut lines = input.lines();

    let instruction = extract_instruction(&mut lines);

    lines.next(); // Skip empty line

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines {
        let (node, direction) = extract_map_item(line);
        map.insert(node, direction);
    }

    let mut step_count = 0;

    let mut current_instruction_index = 0;
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        let (left, right) = map.get(current_node).unwrap();
        if instruction[current_instruction_index] == 'L' {
            current_node = left;
        } else {
            current_node = right;
        }
        current_instruction_index =
            advance_instruction_index(&instruction, current_instruction_index);
        step_count += 1;
    }

    step_count
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_can_extract_instruction() {
        let input = "LR\n\nAAA = (BBB, CCC)";
        let mut lines = input.lines();
        assert_eq!(super::extract_instruction(&mut lines), vec!['L', 'R']);

        let input = "LLLRRL\n\nAAA = (BBB, CCC)";
        let mut lines = input.lines();
        assert_eq!(
            super::extract_instruction(&mut lines),
            vec!['L', 'L', 'L', 'R', 'R', 'L']
        );

        let input = "RRL\n\nAAA = (BBB, CCC)";
        let mut lines = input.lines();
        assert_ne!(super::extract_instruction(&mut lines), vec!['R', 'R', 'R']);
    }

    #[test]
    fn it_can_extract_map_item() {
        let input = "AAA = (BBB, CCC)";
        let (node, (left, right)) = super::extract_map_item(input);
        assert_eq!(node, "AAA");
        assert_eq!(left, "BBB");
        assert_eq!(right, "CCC");

        let input = "FFF = (XZT, ZZZ)";
        let (node, (left, right)) = super::extract_map_item(input);
        assert_eq!(node, "FFF");
        assert_eq!(left, "XZT");
        assert_eq!(right, "ZZZ");
    }

    #[test]
    fn it_can_advance_instruction_index() {
        let instruction = vec!['L', 'R'];
        let mut current_instruction_index = 0;
        current_instruction_index =
            super::advance_instruction_index(&instruction, current_instruction_index);
        assert_eq!(current_instruction_index, 1);

        current_instruction_index =
            super::advance_instruction_index(&instruction, current_instruction_index);
        assert_eq!(current_instruction_index, 0);
        current_instruction_index =
            super::advance_instruction_index(&instruction, current_instruction_index);
        assert_eq!(current_instruction_index, 1);
    }
}
