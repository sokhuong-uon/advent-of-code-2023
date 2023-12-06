use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day02/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", p1(&contents))
}

fn extract_game_data(game: &str) -> (u32, Vec<(u32, u32, u32)>) {
    let game_data = game.split(":").collect::<Vec<&str>>();

    let game_id = game_data[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let sets = game_data[1]
        .split(";")
        .map(process_set)
        .collect::<Vec<(u32, u32, u32)>>();

    (game_id, sets)
}

fn process_set(set: &str) -> (u32, u32, u32) {
    let mut rgb = (0, 0, 0);

    set.trim().split(",").for_each(|cube| {
        let cube_data = cube.trim().split(" ").collect::<Vec<&str>>();
        match cube_data[1] {
            "red" => rgb.0 = cube_data[0].parse::<u32>().unwrap(),
            "green" => rgb.1 = cube_data[0].parse::<u32>().unwrap(),
            "blue" => rgb.2 = cube_data[0].parse::<u32>().unwrap(),
            _ => (),
        }
    });

    rgb
}

fn p1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut rgb = (0, 0, 0);

        let game_data = extract_game_data(line);

        for set in game_data.1 {
            if set.0 > rgb.0 {
                rgb.0 = set.0;
            }
            if set.1 > rgb.1 {
                rgb.1 = set.1;
            }
            if set.2 > rgb.2 {
                rgb.2 = set.2;
            }
        }

        let mut multiply = 1;
        if rgb.0 > 0 {
            multiply *= rgb.0;
        }
        if rgb.1 > 0 {
            multiply *= rgb.1;
        }
        if rgb.2 > 0 {
            multiply *= rgb.2;
        }
        sum += multiply;
    }

    sum
}
