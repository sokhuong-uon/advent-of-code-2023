use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day15/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

fn get_nidividual_step(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

fn hash(step: &str) -> u32 {
    let mut current_value = 0;

    for char in step.chars() {
        current_value += char as u32;
        current_value *= 17;
        current_value = current_value % 256;
    }

    current_value
}

fn solution(input: &str) -> u32 {
    let steps = get_nidividual_step(input);
    steps.iter().map(|step| hash(step)).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_hash() {
        let step = "HASH";
        let hashed = hash(step);
        assert_eq!(hashed, 52);
    }
}
