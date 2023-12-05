use std::fs;

enum DataType {
    SeedToSoil,
    SoilToFert,
    FertToWater,
    WaterToLight,
    LightToTemp,
    TempToHumid,
    HumidToLocation,
    Separator,
    MapData([u64; 3]),
}

struct MapGroup {
    cat: DataType,
    data: [u64; 3],
}

fn sort_items (item: &str) -> DataType {
    match item {
        "seed-to-soil map:" => DataType::SeedToSoil,
        "soil-to-fertilizer map:" => DataType::SoilToFert,
        "fertilizer-to-water map:" => DataType::FertToWater,
        "water-to-light map:" => DataType::WaterToLight,
        "light-to-temperature map:" => DataType::LightToTemp,
        "temperature-to-humidity map:" => DataType::TempToHumid,
        "humidity-to-location map:" => DataType::HumidToLocation,
        "" => DataType::Separator,
        _ => DataType::MapData({
            item.split(" ")
                .into_iter()
                .take(3)
                .map(|x| 
                    x.parse::<u64>().unwrap()
                )
                .collect::<Vec<u64>>()
                .try_into().unwrap()
        }),
    }
}

fn main() {
    let file_path = "./testCase.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let list: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
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

    for seed in seeds {
        println!("Seed: {seed}");
    }

    // unsorted array
    let item_array: Vec<DataType> = list
        .into_iter()
        .skip(2) // skip the seeds list and the empty line after that
        .map(|x| {
            println!("X: {x}");
            sort_items(x.as_str())
        })
        .collect();

    for item in item_array {

    }
}
