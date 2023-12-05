use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./src/bin/in.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("total: {}", solution(&contents));
}

fn collect_seed(line: &str) -> Vec<u32> {
    line.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .filter_map(|numeric_string| numeric_string.parse::<u32>().ok())
        .collect::<Vec<u32>>()
}

fn populate_map(map: &mut Map, line: &str) {
    let map_numbers = line
        .split(" ")
        .map(|numeric_string| numeric_string.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for i in 0..map_numbers[2] {
        map.insert(map_numbers[1] + i, map_numbers[0] + i);
    }
}

struct Maps {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

fn find_location_for_seed(maps: &Maps, seed: &u32) -> u32 {
    let soil = maps.seed_to_soil.get(&seed).unwrap_or_else(|| &seed);

    let fertilizer = maps.soil_to_fertilizer.get(soil).unwrap_or_else(|| soil);

    let water = maps
        .fertilizer_to_water
        .get(fertilizer)
        .unwrap_or_else(|| fertilizer);

    let light = maps.water_to_light.get(water).unwrap_or_else(|| water);

    let temperature = maps
        .light_to_temperature
        .get(light)
        .unwrap_or_else(|| light);

    let humidity = maps
        .temperature_to_humidity
        .get(temperature)
        .unwrap_or_else(|| temperature);

    let location = maps
        .humidity_to_location
        .get(humidity)
        .unwrap_or_else(|| humidity);

    *location
}

type Map = HashMap<u32, u32>;

fn solution(input: &str) -> u32 {
    let mut seeds: Vec<u32> = vec![];

    let mut maps = Maps {
        seed_to_soil: HashMap::new(),
        soil_to_fertilizer: HashMap::new(),
        fertilizer_to_water: HashMap::new(),
        water_to_light: HashMap::new(),
        light_to_temperature: HashMap::new(),
        temperature_to_humidity: HashMap::new(),
        humidity_to_location: HashMap::new(),
    };

    let mut should_populate_map_on_next_iteration = false;

    let mut current_map = &mut maps.seed_to_soil;

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
            populate_map(&mut current_map, line);
            continue;
        }

        if line == "seed-to-soil map:" {
            current_map = &mut maps.seed_to_soil;
            should_populate_map_on_next_iteration = true;
        } else if line == "soil-to-fertilizer map:" {
            current_map = &mut maps.soil_to_fertilizer;
            should_populate_map_on_next_iteration = true;
        } else if line == "fertilizer-to-water map:" {
            current_map = &mut maps.fertilizer_to_water;
            should_populate_map_on_next_iteration = true;
        } else if line == "water-to-light map:" {
            current_map = &mut maps.water_to_light;
            should_populate_map_on_next_iteration = true;
        } else if line == "light-to-temperature map:" {
            current_map = &mut maps.light_to_temperature;
            should_populate_map_on_next_iteration = true;
        } else if line == "temperature-to-humidity map:" {
            current_map = &mut maps.temperature_to_humidity;
            should_populate_map_on_next_iteration = true;
        } else if line == "humidity-to-location map:" {
            current_map = &mut maps.humidity_to_location;
            should_populate_map_on_next_iteration = true;
        }
    }

    let might = seeds
        .iter()
        .map(|seed| find_location_for_seed(&maps, seed))
        .fold(u32::MAX, |acc, v| if v < acc { v } else { acc });

    println!("might: {:?}", might);

    might
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
        let mut map = super::Map::new();
        let line = "50 98 2";

        super::populate_map(&mut map, line);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&98), Some(&50));
        assert_eq!(map.get(&99), Some(&51));

        //
        let mut map = super::Map::new();
        let line = "52 50 48";

        super::populate_map(&mut map, line);

        assert_eq!(map.len(), 48);
        assert_eq!(map.get(&50), Some(&52));
        assert_eq!(map.get(&51), Some(&53));
        for i in 50..98 {
            assert_eq!(map.get(&i), Some(&(i + 2)));
        }

        //
        let mut map = super::Map::new();
        let line = "50 98 100";

        super::populate_map(&mut map, line);

        assert_eq!(map.len(), 100);
        assert_eq!(map.get(&98), Some(&50));
        assert_eq!(map.get(&99), Some(&51));
        assert_eq!(map.get(&100), Some(&52));
        for i in 98..198 {
            assert_eq!(map.get(&i), Some(&(i - 48)));
        }
    }

    #[test]
    fn it_can_find_location_for_seed() {
        let mut maps = super::Maps {
            seed_to_soil: super::Map::new(),
            soil_to_fertilizer: super::Map::new(),
            fertilizer_to_water: super::Map::new(),
            water_to_light: super::Map::new(),
            light_to_temperature: super::Map::new(),
            temperature_to_humidity: super::Map::new(),
            humidity_to_location: super::Map::new(),
        };

        let seed = 3;

        maps.seed_to_soil.insert(3, 1);
        maps.seed_to_soil.insert(2, 5);

        maps.soil_to_fertilizer.insert(1, 2);
        maps.soil_to_fertilizer.insert(5, 100);

        maps.fertilizer_to_water.insert(2, 15);
        maps.fertilizer_to_water.insert(45, 4);

        maps.water_to_light.insert(3, 4);
        maps.water_to_light.insert(15, 8);

        maps.light_to_temperature.insert(4, 5);
        maps.light_to_temperature.insert(34, 99);

        maps.temperature_to_humidity.insert(5, 6);
        maps.temperature_to_humidity.insert(8, 25);

        maps.humidity_to_location.insert(6, 7);
        maps.humidity_to_location.insert(32, 9);
        maps.humidity_to_location.insert(100, 69);

        let location = super::find_location_for_seed(&maps, &seed);
        assert_eq!(location, 25);

        let seed = 2;
        let location = super::find_location_for_seed(&maps, &seed);
        assert_eq!(location, 69);
    }
}
