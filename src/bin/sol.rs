use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./src/bin/in.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", p1(&contents));
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
    let rgb = (12, 13, 14);

    for line in input.lines() {
        let game_data = extract_game_data(line);

        let mut possible = true;

        for set in game_data.1 {
            if set.0 > rgb.0 || set.1 > rgb.1 || set.2 > rgb.2 {
                possible = false;
            }
        }

        if possible {
            sum += game_data.0;
        }
    }

    sum
}
