use std::collections::BTreeMap;

pub mod part1;
pub mod part2;

pub struct Farming {
    seeds: Vec<u64>,
    seed_to_soil: BTreeMap<u64, u64>,
    soil_to_fertilizer: BTreeMap<u64, u64>,
    fertilizer_to_water: BTreeMap<u64, u64>,
    water_to_light: BTreeMap<u64, u64>,
    light_to_temp: BTreeMap<u64, u64>,
    temp_to_humidity: BTreeMap<u64, u64>,
    humidity_to_location: BTreeMap<u64, u64>,
}