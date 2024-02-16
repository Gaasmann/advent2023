use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod farm_map;

fn build_seed_list(lines: &mut impl Iterator<Item = String>) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut seeds_str = None;
    for line in lines {
        if line.contains("seeds:") {
            let (_, res) = line.split_once(": ").ok_or(ParseError::MalformedSeeds)?;
            seeds_str = Some(res.to_string());
            break;
        }
    }
    match seeds_str {
        Some(string) => {
            Ok(string.split(' ').map(|v| v.parse().unwrap()).collect()) // TODO catch error in
                                                                        // maps?
        }
        None => Err(Box::new(ParseError::MalformedSeeds)),
    }
}

fn build_map(
    lines: &mut impl Iterator<Item = String>,
    map_type: &str,
) -> Result<farm_map::Map, Box<dyn Error>> {
    let title = format!("{} map:", map_type);
    let mut found = false;
    for line in &mut *lines {
        if line == title {
            found = true;
            break;
        }
    }
    if !found {
        return Err(Box::new(ParseError::MapNotFound));
    }
    let mut map = farm_map::Map::new();
    for line in &mut *lines {
        if line.is_empty() {
            break;
        }
        map.add_entry(line.parse()?);
    }
    map.sort();
    Ok(map)
}

fn open_and_read_lines(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.unwrap())
}

#[derive(Debug)]
enum ParseError {
    MalformedSeeds,
    MapNotFound,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Oh no => {:?}", self)
    }
}
impl Error for ParseError {}

fn main() -> Result<(), Box<dyn Error>> {
    let mut iter = open_and_read_lines("input.txt");
    let seeds = build_seed_list(&mut iter)?;
    println!("{:#?}", seeds);
    // TODO Build an almanac struct to store the map and exec the find nearest location
    let seed_to_soil = build_map(&mut iter, "seed-to-soil")?;
    let soil_to_fertilizer = build_map(&mut iter, "soil-to-fertilizer")?;
    let fertilizer_to_water = build_map(&mut iter, "fertilizer-to-water")?;
    let water_to_light = build_map(&mut iter, "water-to-light")?;
    let light_to_temperature = build_map(&mut iter, "light-to-temperature")?;
    let temperature_to_humidity = build_map(&mut iter, "temperature-to-humidity")?;
    let humidity_to_location = build_map(&mut iter, "humidity-to-location")?;

    let result = seeds
        .iter()
        .map(|seed| seed_to_soil.convert(*seed).unwrap())
        .map(|soil| soil_to_fertilizer.convert(soil).unwrap())
        .map(|fertilizer| fertilizer_to_water.convert(fertilizer).unwrap())
        .map(|water| water_to_light.convert(water).unwrap())
        .map(|light| light_to_temperature.convert(light).unwrap())
        .map(|temperature| temperature_to_humidity.convert(temperature).unwrap())
        .map(|humidity| humidity_to_location.convert(humidity).unwrap())
        .min();
    println!("The result to part 1 is {}", result.unwrap());

    // Part 2
    let result2 = seeds
        .chunks(2)
        .flat_map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
        .map(|seed| seed_to_soil.convert(seed).unwrap())
        .map(|soil| soil_to_fertilizer.convert(soil).unwrap())
        .map(|fertilizer| fertilizer_to_water.convert(fertilizer).unwrap())
        .map(|water| water_to_light.convert(water).unwrap())
        .map(|light| light_to_temperature.convert(light).unwrap())
        .map(|temperature| temperature_to_humidity.convert(temperature).unwrap())
        .map(|humidity| humidity_to_location.convert(humidity).unwrap())
        .min(); // TODO slow ~190s
    println!("The result to part 2 is {:?}", result2.unwrap());

    Ok(())
}
