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

struct MapGroup {
    cat: String,
    data: Vec<MapDoc>,
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

fn map(seed: u64, mapList: Vec<MapDoc>) -> u64 {
    let mut result = seed;
    for map in mapList {
        if seed >= map[1] && seed < map[1]+map[2] {
            result = map[0] + seed - map[1]
        }
    }
    result
}


fn main() {
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let map_strings: Vec<String> = contents.split("\n\n").map(|x| x.to_string()).collect();

    // convert map strings to list of MapGroups
    let raw_map_groups: Vec<RawMapGroup> = map_strings
        .iter()
        .skip(1)
        .map(|s| {
            let map_type = s.split(":").next().unwrap().to_string();
            let map_data = s.split(":").skip(1).next().unwrap().to_string();
            println!("map data: {map_data}");

            let map_group: RawMapGroup = RawMapGroup { cat: map_type, data: map_data };
            map_group
        }).collect();

    let map_groups: Vec<Map> = raw_map_groups.iter().map(|map_group| {
        let cat = map_group.cat.clone();
        println!("data: {}", map_group.data);
        let data_strings: Vec<String> = if let Some(x) = map_group.data.strip_prefix("\n\n")
        {
            x.split("\n")
            .map(|s| s.to_string())
            .collect()
        } else {
            map_group.data
            .split("\n")
            // .split_whitespace()
            .map(|s| s.to_string())
            .collect()
        };
        
        let data: Vec<MapDoc> = data_strings.iter().map(|s| {
            if s == "" {
                None
            } else {
                let vec: Vec<u64> = s.split_whitespace()
                    .map(|x| x.to_string()
                        .parse::<u64>()
                        .unwrap())
                    .collect();
                let arr: MapDoc = [vec[0], vec[1], vec[2]];
                Some(arr) 
            }
        })
        .filter_map(|x| x)
        .collect();
        
        Map { cat: sort_items(cat.as_str()), data: data }
    }).collect();


    let list: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    // problem one
    let seeds: Vec<u64> = if let Some(x) = list[0].strip_prefix("seeds: ") {
        x.split_whitespace()
            .flat_map(str::parse::<u64>)
            .collect()
    } else {
        list[0]
            .split_whitespace()
            .flat_map(str::parse::<u64>)
            .collect()
    };

    let mut lowest: u64 = u64::MAX;
    for seed in seeds {
        // println!("Seed: {seed}
        let soil = map(seed, map_groups[0].data.clone());
        let fert = map(soil, map_groups[1].data.clone());
        let water = map(fert, map_groups[2].data.clone());
        let light = map(water, map_groups[3].data.clone());
        let temp = map(light, map_groups[4].data.clone());
        let humid = map(temp, map_groups[5].data.clone());
        let location = map(humid, map_groups[6].data.clone());
        if location < lowest {
            lowest = location
        }
        println!("seed: {}, location: {}", seed, location);
    }
    println!("lowest: {}", lowest);


    // println!("pls be 58: {}", map(95, map_groups[0].data.clone()));
    // // unsorted array
    // let item_array: Vec<DataType> = list
    //     .into_iter()
    //     .skip(2) // skip the seeds list and the empty line after that
    //     .map(|x| {
    //         println!("X: {x}");
    //         sort_items(x.as_str())
    //     })
    //     .collect();
    //
    // for item in item_array {
    //
    // }
}
