use std::collections::HashMap;
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

fn get_label_and_focal_length(step: &str) -> (&str, Option<u8>) {
    if step.ends_with('-') {
        return (&step[..(step.len() - 1)], None);
    } else {
        let mut parts = step.split("=");
        let label = parts.clone().nth(0).unwrap();
        let focal_length = parts.nth(1).unwrap().parse::<u8>().unwrap();
        return (label, Some(focal_length));
    }
}

fn hash(step: &str) -> usize {
    let mut current_value = 0;

    for char in step.chars() {
        current_value += char as usize;
        current_value *= 17;
        current_value = current_value % 256;
    }

    current_value
}

fn solution(input: &str) -> u32 {
    let steps = get_nidividual_step(input);

    let mut boxes: Vec<Option<(u32, HashMap<&str, (u8, u32)>)>> = vec![];
    for _ in 0..256 {
        boxes.push(None);
    }

    steps.iter().for_each(|step| {
        let (label, focal_length) = get_label_and_focal_length(step);
        let hash = hash(label);

        if let Some(focal_length) = focal_length {
            if boxes[hash].is_none() {
                boxes[hash] = Some((0, HashMap::new()));

                let (last_order, lens) = boxes[hash].as_mut().unwrap();
                lens.insert(label, (focal_length, *last_order + 1));
                *last_order += 1;
            } else {
                let (last_order, lens) = boxes[hash].as_mut().unwrap();

                if let Some(focal_and_order) = lens.get_mut(label) {
                    focal_and_order.0 = focal_length;
                } else {
                    lens.insert(label, (focal_length, *last_order + 1));
                    *last_order += 1;
                }
            }
        } else {
            if boxes[hash].is_some() {
                let (last_order, lens) = boxes[hash].as_mut().unwrap();

                if let Some(focal_and_order) = lens.clone().get(label) {
                    lens.remove(label);

                    lens.iter_mut().for_each(|(_, (_, order))| {
                        if *order > focal_and_order.1 {
                            *order -= 1;
                        }
                    });

                    *last_order -= 1;
                }
            }
        }
    });

    boxes.iter().enumerate().fold(0, |acc, (index, boxes)| {
        let mut per_box = 0;

        if let Some(lens) = boxes {
            lens.1.iter().for_each(|(_, (focal_length, order))| {
                per_box += *focal_length as u32 * (index as u32 + 1) * order;
            })
        }
        per_box + acc
    })
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
