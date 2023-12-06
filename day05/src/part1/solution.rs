use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day05/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    format!("Closest Location: {}", solution(&contents))
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

struct Maps {
    seed_to_soil: Vec<MapItem>,
    soil_to_fertilizer: Vec<MapItem>,
    fertilizer_to_water: Vec<MapItem>,
    water_to_light: Vec<MapItem>,
    light_to_temperature: Vec<MapItem>,
    temperature_to_humidity: Vec<MapItem>,
    humidity_to_location: Vec<MapItem>,
}

fn solution(input: &str) -> u64 {
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
        println!("line: {:?}", line);
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

    seeds
        .iter()
        .map(|seed| find_location_from_seed(&maps, seed))
        .fold(u64::MAX, |acc, v| if v < acc { v } else { acc })
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
}
