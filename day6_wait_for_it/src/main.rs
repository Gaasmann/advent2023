use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod race;

fn read_races(filename: &str) -> Vec<race::Race> {
    // TODO handle error better
    let data_file = File::open(filename).unwrap();
    let buf = BufReader::new(data_file);
    let mut iter = buf.lines();
    let time_str = iter.next().unwrap().unwrap();
    let distance_str = iter.next().unwrap().unwrap();
    //time
    let (tag, values_str) = time_str.split_once(':').unwrap();
    if tag != "Time" {
        panic!("Not the Time line you're looking for")
    }
    static REGEX_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    let race_times = REGEX_NUMBER
        .find_iter(values_str)
        .map(|m| m.as_str())
        .map(|s| s.parse::<u64>().unwrap());
    //distance
    //TODO refactor the dup on number extraction
    let (tag, values_str) = distance_str.split_once(':').unwrap();
    if tag != "Distance" {
        panic!("Not the Distance line you're looking for")
    }
    let race_distance = REGEX_NUMBER
        .find_iter(values_str)
        .map(|m| m.as_str())
        .map(|s| s.parse::<u64>().unwrap());
    race_times
        .zip(race_distance)
        .map(|(t, d)| race::Race::new(t, d))
        .collect()
}

// TODO lot of dup with above func
// TODO rework the func (no need for regex, no need to zip/collect, ...)
fn read_as_single_race(filename: &str) -> race::Race {
    // TODO handle error better
    let data_file = File::open(filename).unwrap();
    let buf = BufReader::new(data_file);
    let mut iter = buf.lines();
    let time_str = iter.next().unwrap().unwrap();
    let distance_str = iter.next().unwrap().unwrap();
    //time
    let (tag, values_str) = time_str.split_once(':').unwrap();
    if tag != "Time" {
        panic!("Not the Time line you're looking for")
    }
    static REGEX_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    let value_str = values_str.replace(' ', "");
    let race_times = REGEX_NUMBER
        .find_iter(value_str.as_str())
        .map(|m| m.as_str())
        .map(|s| s.parse::<u64>().unwrap());
    //distance
    let (tag, values_str) = distance_str.split_once(':').unwrap();
    if tag != "Distance" {
        panic!("Not the Distance line you're looking for")
    }
    let value_str = values_str.replace(' ', "");
    let race_distance = REGEX_NUMBER
        .find_iter(value_str.as_str())
        .map(|m| m.as_str())
        .map(|s| s.parse::<u64>().unwrap());
    let mut result: Vec<_> = race_times
        .zip(race_distance)
        .map(|(t, d)| race::Race::new(t, d))
        .collect();
    result.pop().unwrap()
}

fn main() {
    let races = read_races("input.txt");
    // part 1
    let result: usize = races
        .iter()
        .map(|race| race.find_solutions_to_beat_record())
        .map(|solutions| solutions.len())
        .product();
    println!("The solution for the part 1 is: {}", result);
    // part 2
    let race = read_as_single_race("input.txt");
    let (lower, upper) = race.find_solution_bounds_to_beat_record();
    let result = (lower..upper).count();
    println!("The solution for the part 2 is: {}", result)
}
