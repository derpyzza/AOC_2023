use std::fs;

type MapDoc = [u64; 3];
enum DataType {
    SeedToSoil,
    SoilToFert,
    FertToWater,
    WaterToLight,
    LightToTemp,
    TempToHumid,
    HumidToLocation,
    None
}

struct RawMapGroup {
    cat: String,
    data: String,
}

struct Map {
    cat: DataType,
    data: Vec<MapDoc>,
}

fn sort_items (item: &str) -> DataType {
    match item {
        "seed-to-soil map" => DataType::SeedToSoil,
        "soil-to-fertilizer map" => DataType::SoilToFert,
        "fertilizer-to-water map" => DataType::FertToWater,
        "water-to-light map" => DataType::WaterToLight,
        "light-to-temperature map" => DataType::LightToTemp,
        "temperature-to-humidity map" => DataType::TempToHumid,
        "humidity-to-location map" => DataType::HumidToLocation,
        _ => DataType::None,
    }
}

fn map(seed: u64, mapList: &Vec<MapDoc>) -> u64 {
    let mut result = seed;
    for map in mapList {
        if seed >= map[1] && seed < map[1]+map[2] {
            result = map[0] + seed - map[1]
        }
    }
    result
}

fn map_to_map(seed: u64, mapList: &Vec<MapDoc>) -> u64 {
    let mut result = seed;
    for map in mapList {
        if seed >= map[0] && seed < map[0]+map[2] {
            result = map[1] + seed - map[0]
        }
    }
    result
}

fn get_data(contents: &str) -> (Vec<u64>, Vec<String>){
    // splits input into strings at every empty line:
    // 0: 'seed: 23 23 123 45 21', 1: "seed-to-soil map:\n 23 23 35\n234 13 56\n34 465 12" etc.
    let blocks_list: Vec<String> = contents.split("\n\n").map(|x| x.to_string()).collect();
    println!("list: {:#?}", blocks_list);

    let seeds: Vec<u64> = if let Some(x) = blocks_list[0].strip_prefix("seeds: ") {
        x.split_whitespace()
            .flat_map(str::parse::<u64>)
            .collect()
    } else {
        blocks_list[0]
            .split_whitespace()
            .flat_map(str::parse::<u64>)
            .collect()
    };

    let list = blocks_list.iter().skip(1).map(|x| x.clone()).collect();
    (seeds, list)
}

// 50, 98, 2\n
fn string_to_mapdoc (data: String) -> MapDoc {
    let result: MapDoc = data.split_whitespace()
        .into_iter()
        .map(|s| {
            s.to_string().parse::<u64>().unwrap()
        }).collect::<Vec<_>>().try_into().unwrap();
    result
}

fn extract_maps(content: Vec<String>) -> Vec<Vec<MapDoc>> {
    let res: Vec<Vec<MapDoc>> = content.iter()
        .map(|s| {
            s.split("\n")
                .skip(1)
                .map(|s|{
                    if s==""{
                        None
                    } else {
                        Some(string_to_mapdoc(s.to_string()))
                    }
                }).filter_map(|x| x).collect()
        }).collect();

        res
}

fn first_problem(contents: &str) -> u64 {
    let (seeds, maps) = get_data(contents);
    let maps_list = extract_maps(maps);
    let mut lowest: u64 = u64::MAX;

    println!("str: {:#?}", string_to_mapdoc("50 98 2\n".to_string()));

    for seed in seeds {
        let soil = map(seed, &maps_list[0]);
        let fert = map(soil, &maps_list[1]);
        let water = map(fert, &maps_list[2]);
        let light = map(water, &maps_list[3]);
        let temp = map(light, &maps_list[4]);
        let humid = map(temp, &maps_list[5]);
        let location = map(humid, &maps_list[6]);

        if location < lowest {
            lowest = location
        }
    }
    lowest
}

fn second_problem(contents: &str) -> u64 {
    let (_, maps) = get_data(contents);
    let maps_list = extract_maps(maps);
    let mut lowest: u64 = u64::MAX;

    println!("str: {:#?}", string_to_mapdoc("50 98 2\n".to_string()));

    for location in 0..u64::MAX  {
        let humid = map_to_map(location, &maps_list[6]);
        let temp = map_to_map(humid, &maps_list[5]);
        let light = map_to_map(temp, &maps_list[4]);
        let water = map_to_map(light, &maps_list[3]);
        let fert = map_to_map(water, &maps_list[2]);
        let soil = map_to_map(fert, &maps_list[1]);
        let seed = map_to_map(soil, &maps_list[0]);
        println!("location: {}, seed: {}", location, seed);
        if seed < lowest {
            lowest = seed
        }
    }
    lowest
}

fn main() {
    let file_path = "./testCase.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // let lowest = first_problem(&contents);
    let lowest = second_problem(&contents);

    println!("lowest: {}", lowest);
 }
