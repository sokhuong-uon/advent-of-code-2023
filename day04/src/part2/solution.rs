use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day04/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    format!("Total card: {}", solution(&contents))
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

fn winning_per_card(card: &str) -> u32 {
    let mut winning = 0;

    let numeric_line = card.split(":").collect::<Vec<&str>>()[1];
    let number_group = get_number_group(numeric_line);

    number_group.1.iter().for_each(|number| {
        if number_group.0.contains_key(number) {
            winning += 1;
        }
    });

    winning
}

fn get_card_instance_if_exists_or_else_add(card_map: &mut CardMap, card_number: u32) -> u32 {
    if let Some(instances) = card_map.get_mut(&(card_number as u32)) {
        return *instances;
    } else {
        card_map.insert(card_number as u32, 1);
        return 1;
    }
}

fn copy_winning_card_per_card(card_map: &mut CardMap, card_number: u32, winning: u32) {
    for winning_card_index in (card_number + 1)..=(card_number + winning) {
        if let Some(instances) = card_map.get_mut(&(winning_card_index as u32)) {
            *instances += 1;
        } else {
            card_map.insert(winning_card_index as u32, 2);
        }
    }
}

type CardMap = HashMap<u32, u32>;

fn solution(input: &str) -> u32 {
    let mut instances = 0;

    let mut card_map: CardMap = HashMap::new();

    for (card_number, card) in input.lines().enumerate() {
        let instances_of_current_card =
            get_card_instance_if_exists_or_else_add(&mut card_map, card_number as u32);
        instances += instances_of_current_card;

        let winning = winning_per_card(card);

        for _ in 0..instances_of_current_card {
            copy_winning_card_per_card(&mut card_map, card_number as u32, winning);
        }
    }

    instances
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn it_could_count_winning() {
        let card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let winning = super::winning_per_card(card);
        assert_eq!(winning, 4);

        let card = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let winning = super::winning_per_card(card);
        assert_eq!(winning, 2);

        let card = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let winning = super::winning_per_card(card);
        assert_eq!(winning, 2);

        let card = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let winning = super::winning_per_card(card);
        assert_eq!(winning, 1);
    }

    #[test]
    fn it_updates_map() {
        let mut card_map = HashMap::new();
        let card_number = 0;

        let instances = super::get_card_instance_if_exists_or_else_add(&mut card_map, card_number);
        assert_eq!(instances, 1);

        let instances = super::get_card_instance_if_exists_or_else_add(&mut card_map, card_number);
        assert_eq!(instances, 1);

        let instances = super::get_card_instance_if_exists_or_else_add(&mut card_map, card_number);
        assert_eq!(instances, 1);
    }

    #[test]
    fn it_copies_card() {
        let mut card_map = HashMap::new();
        let card_number = 3;
        let winning = 4;

        super::copy_winning_card_per_card(&mut card_map, card_number, winning);
        assert_eq!(card_map.contains_key(&card_number), false);

        assert_eq!(*card_map.get(&4).unwrap(), 2);
        assert_eq!(*card_map.get(&5).unwrap(), 2);
        assert_eq!(*card_map.get(&6).unwrap(), 2);
        assert_eq!(*card_map.get(&7).unwrap(), 2);

        let card_number = 4;
        let winning = 2;

        super::copy_winning_card_per_card(&mut card_map, card_number, winning);

        assert_eq!(*card_map.get(&5).unwrap(), 3);
        assert_eq!(*card_map.get(&6).unwrap(), 3);
        assert_eq!(*card_map.get(&7).unwrap(), 2);
    }
}
