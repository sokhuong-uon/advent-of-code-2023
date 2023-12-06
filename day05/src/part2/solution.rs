use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;

use tokio::sync::Mutex;

#[tokio::main]
pub async fn main() -> String {
    let args: Vec<String> = std::env::args().collect();
    let dir = std::env::current_dir().unwrap();

    if args.len() < 2 {
        let mut file = File::open(format!("{}/day05/src/in.txt", dir.display())).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let solution = solution(&contents).await;
        return format!("Closest location: {}", solution);
    }

    let file_name_without_ext = &args[1];
    let file_path = format!("{}/day05/src/{}.txt", dir.display(), file_name_without_ext);

    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let solution = solution(&contents).await;

    let mut output_file = File::options()
        .append(true)
        .open(format!("{}/day05/src/out.txt", dir.display()))
        .unwrap();
    writeln!(output_file, "\n{}", solution).unwrap();

    format!("Closest location: {}", solution)
}

fn collect_seed(line: &str) -> Vec<u64> {
    line.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .filter_map(|numeric_string| numeric_string.parse::<u64>().ok())
        .collect::<Vec<u64>>()
}

fn populate_map(map: &mut Vec<MapItem>, line: &str) {
    let map_item = line
        .split(" ")
        .map(|numeric_string| numeric_string.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    map.push(MapItem::new(map_item[0], map_item[1], map_item[2]));
}

fn find_location_from_seed(maps: &Maps, seed: &u64) -> u64 {
    let mut location = *seed;
    location = find_destination(&maps.seed_to_soil, &location);
    location = find_destination(&maps.soil_to_fertilizer, &location);
    location = find_destination(&maps.fertilizer_to_water, &location);
    location = find_destination(&maps.water_to_light, &location);
    location = find_destination(&maps.light_to_temperature, &location);
    location = find_destination(&maps.temperature_to_humidity, &location);
    location = find_destination(&maps.humidity_to_location, &location);
    location
}

fn find_destination(map: &Vec<MapItem>, source: &u64) -> u64 {
    let mut destination = *source;
    for map_item in map {
        if let Some(d) = map_item.find_destination(source) {
            destination = d;
            break;
        }
    }
    destination
}

#[derive(Clone)]
struct MapItem {
    destination_start: u64,
    source_start: u64,
    range: u64,
}
impl MapItem {
    fn new(destination_start: u64, source_start: u64, range: u64) -> MapItem {
        MapItem {
            destination_start,
            source_start,
            range,
        }
    }

    fn find_destination(&self, source: &u64) -> Option<u64> {
        if self.source_start <= *source && *source <= self.source_start + self.range - 1 {
            let extra = *source - self.source_start;
            Some(self.destination_start + extra)
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Maps {
    seed_to_soil: Vec<MapItem>,
    soil_to_fertilizer: Vec<MapItem>,
    fertilizer_to_water: Vec<MapItem>,
    water_to_light: Vec<MapItem>,
    light_to_temperature: Vec<MapItem>,
    temperature_to_humidity: Vec<MapItem>,
    humidity_to_location: Vec<MapItem>,
}

async fn find_closest_location_from_seeds_range(
    map: Arc<Mutex<Maps>>,
    start_range: u64,
    length: u64,
) -> u64 {
    let mut closest_location = u64::MAX;

    let map_lock = map.lock().await;
    let current_map = &*map_lock;

    for seed in start_range..(start_range + length) {
        let location = find_location_from_seed(current_map, &seed);
        if location < closest_location {
            closest_location = location;
        }
    }

    closest_location
}

fn divide_range_into_sub_ranges(
    start_range: u64,
    range_length: u64,
    sub_range_length: u64,
) -> Vec<(u64, u64)> {
    let mut sub_ranges = vec![];

    let mut sub_range_start = start_range;
    let mut length = range_length;

    while length > 0 {
        if length < sub_range_length {
            sub_ranges.push((sub_range_start, length));
            break;
        }

        sub_ranges.push((sub_range_start, sub_range_length));
        sub_range_start += sub_range_length;
        length -= sub_range_length;
    }

    sub_ranges
}

fn divide_seed_ranges_into_sub_seed_ranges(
    seeds: Vec<u64>,
    sub_range_length: u64,
) -> Vec<(u64, u64)> {
    let mut sub_ranges = vec![];
    for i in (0..seeds.len()).step_by(2) {
        divide_range_into_sub_ranges(seeds[i], seeds[i + 1], sub_range_length)
            .iter()
            .for_each(|sub_range| {
                sub_ranges.push(*sub_range);
            })
    }
    sub_ranges
}

async fn solution(input: &str) -> u64 {
    let mut seeds: Vec<u64> = vec![];

    let mut maps = Maps {
        seed_to_soil: vec![],
        soil_to_fertilizer: vec![],
        fertilizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    };

    let mut current_map = &mut maps.seed_to_soil;

    let mut should_populate_map_on_next_iteration = false;

    for line in input.lines() {
        if line.is_empty() {
            should_populate_map_on_next_iteration = false;
            continue;
        }

        if line.starts_with("seeds:") {
            seeds = collect_seed(line);
            continue;
        }

        if should_populate_map_on_next_iteration {
            populate_map(current_map, line);
            continue;
        }

        current_map = match line {
            "seed-to-soil map:" => &mut maps.seed_to_soil,
            "soil-to-fertilizer map:" => &mut maps.soil_to_fertilizer,
            "fertilizer-to-water map:" => &mut maps.fertilizer_to_water,
            "water-to-light map:" => &mut maps.water_to_light,
            "light-to-temperature map:" => &mut maps.light_to_temperature,
            "temperature-to-humidity map:" => &mut maps.temperature_to_humidity,
            "humidity-to-location map:" => &mut maps.humidity_to_location,
            _ => panic!("unknown map"),
        };
        should_populate_map_on_next_iteration = true;
    }

    let current_map = Arc::new(Mutex::new(Maps {
        seed_to_soil: maps.seed_to_soil.clone(),
        soil_to_fertilizer: maps.soil_to_fertilizer.clone(),
        fertilizer_to_water: maps.fertilizer_to_water.clone(),
        water_to_light: maps.water_to_light.clone(),
        light_to_temperature: maps.light_to_temperature.clone(),
        temperature_to_humidity: maps.temperature_to_humidity.clone(),
        humidity_to_location: maps.humidity_to_location.clone(),
    }));

    let sub_ranges = divide_seed_ranges_into_sub_seed_ranges(seeds, 100_000);

    let mut tasks = vec![];
    for (sub_range_start, sub_range_length) in sub_ranges.iter() {
        let current_map_clone = Arc::clone(&current_map);
        tasks.push(tokio::spawn(find_closest_location_from_seeds_range(
            current_map_clone,
            *sub_range_start,
            *sub_range_length,
        )));
    }

    let mut closest_locations = vec![];

    for task in tasks {
        let result = task.await.unwrap();
        closest_locations.push(result);
        println!(
            "Progress: {}%",
            closest_locations.len() * 100 / sub_ranges.len()
        );
    }

    *closest_locations.iter().min().unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_can_collect_seed() {
        let line = "seeds: 1 2 3 4 5";
        let seeds = super::collect_seed(line);
        assert_eq!(seeds, vec![1, 2, 3, 4, 5]);

        let line = "seeds: 79 14 55 13";
        let seeds = super::collect_seed(line);
        assert_eq!(seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn it_can_populate_map() {
        let mut map = vec![];
        let line = "1 2 3";

        super::populate_map(&mut map, line);
        assert_eq!(map[0].destination_start, 1);
        assert_eq!(map[0].source_start, 2);
        assert_eq!(map[0].range, 3);

        let line = "50 98 2";
        super::populate_map(&mut map, line);
        assert_eq!(map[1].destination_start, 50);
        assert_eq!(map[1].source_start, 98);
        assert_eq!(map[1].range, 2);
    }

    #[test]
    fn it_can_find_destination_per_map_item() {
        let map_item = super::MapItem::new(50, 98, 2);

        assert_eq!(map_item.find_destination(&97), None);
        assert_eq!(map_item.find_destination(&98), Some(50));
        assert_eq!(map_item.find_destination(&99), Some(51));
        assert_eq!(map_item.find_destination(&100), None);

        let map_item = super::MapItem::new(52, 50, 48);

        assert_eq!(map_item.find_destination(&49), None);
        assert_eq!(map_item.find_destination(&50), Some(52));
        assert_eq!(map_item.find_destination(&51), Some(53));
        assert_eq!(map_item.find_destination(&97), Some(99));
        assert_eq!(map_item.find_destination(&99), None);
    }

    #[test]
    fn it_can_find_destination_per_map() {
        let mut map = vec![];

        let line = "50 98 2";
        super::populate_map(&mut map, line);

        let line = "52 50 48";
        super::populate_map(&mut map, line);

        assert_eq!(super::find_destination(&map, &49), 49);
        assert_eq!(super::find_destination(&map, &50), 52);
        assert_eq!(super::find_destination(&map, &51), 53);
        assert_eq!(super::find_destination(&map, &97), 99);
    }
    #[test]
    fn it_can_find_location_from_seed() {
        let seed = 10;

        let mut maps = super::Maps {
            seed_to_soil: vec![],
            soil_to_fertilizer: vec![],
            fertilizer_to_water: vec![],
            water_to_light: vec![],
            light_to_temperature: vec![],
            temperature_to_humidity: vec![],
            humidity_to_location: vec![],
        };

        maps.seed_to_soil.push(super::MapItem::new(5, 8, 3)); // 10 -> 7
        maps.soil_to_fertilizer.push(super::MapItem::new(10, 5, 5)); // 7 -> 12
        maps.fertilizer_to_water
            .push(super::MapItem::new(15, 10, 5)); // 12 -> 17
        maps.water_to_light.push(super::MapItem::new(50, 80, 100)); // 17 -> 17
        maps.light_to_temperature
            .push(super::MapItem::new(100, 15, 100)); // 17 -> 102
        maps.temperature_to_humidity
            .push(super::MapItem::new(88, 100, 5)); // 102 -> 90
        maps.humidity_to_location
            .push(super::MapItem::new(100, 88, 12)); // 90 -> 102

        let location = super::find_location_from_seed(&maps, &seed);
        assert_eq!(location, 102);
    }

    #[test]
    fn it_can_divide_range_into_sub_ranges() {
        let start_range = 5;
        let range_length = 4;
        let sub_range_length = 1;

        let sub_ranges =
            super::divide_range_into_sub_ranges(start_range, range_length, sub_range_length);
        assert_eq!(sub_ranges, vec![(5, 1), (6, 1), (7, 1), (8, 1),]);

        let start_range = 3;
        let range_length = 9;
        let sub_range_length = 2;

        let sub_ranges =
            super::divide_range_into_sub_ranges(start_range, range_length, sub_range_length);
        assert_eq!(sub_ranges, vec![(3, 2), (5, 2), (7, 2), (9, 2), (11, 1)]);

        let start_range = 15;
        let range_length = 3;
        let sub_range_length = 4;

        let sub_ranges =
            super::divide_range_into_sub_ranges(start_range, range_length, sub_range_length);
        assert_eq!(sub_ranges, vec![(15, 3)]);

        let start_range = 500;
        let range_length = 4;
        let sub_range_length = 4;

        let sub_ranges =
            super::divide_range_into_sub_ranges(start_range, range_length, sub_range_length);
        assert_eq!(sub_ranges, vec![(500, 4)]);
    }

    #[test]
    fn it_can_divide_seed_ranges_into_sub_seed_ranges() {
        let seeds = vec![79, 14, 55, 13];
        let sub_range_length = 1;
        let sub_ranges = super::divide_seed_ranges_into_sub_seed_ranges(seeds, sub_range_length);

        assert_eq!(
            sub_ranges,
            vec![
                (79, 1),
                (80, 1),
                (81, 1),
                (82, 1),
                (83, 1),
                (84, 1),
                (85, 1),
                (86, 1),
                (87, 1),
                (88, 1),
                (89, 1),
                (90, 1),
                (91, 1),
                (92, 1),
                //
                (55, 1),
                (56, 1),
                (57, 1),
                (58, 1),
                (59, 1),
                (60, 1),
                (61, 1),
                (62, 1),
                (63, 1),
                (64, 1),
                (65, 1),
                (66, 1),
                (67, 1),
            ]
        );

        let seeds = vec![79, 14, 55, 13];
        let sub_range_length = 4;
        let sub_ranges = super::divide_seed_ranges_into_sub_seed_ranges(seeds, sub_range_length);

        assert_eq!(
            sub_ranges,
            vec![
                (79, 4),
                (83, 4),
                (87, 4),
                (91, 2),
                //
                (55, 4),
                (59, 4),
                (63, 4),
                (67, 1),
            ]
        )
    }
}
