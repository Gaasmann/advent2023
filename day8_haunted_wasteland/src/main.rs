use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod map;

fn main() {
    static REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap());
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);
    let mut lines = buf.lines();
    // get directions
    let dir_str = lines.next().unwrap().unwrap();
    let directions = map::Direction::build_vec(&dir_str).unwrap();
    let mut map = map::Map::new(directions);
    // get cells
    lines.next();
    for cell_str in lines {
        let haystack = cell_str.unwrap();
        let matches = REGEX.captures(&haystack).unwrap();
        map.add_cell(
            matches.get(1).unwrap().clone().as_str(),
            matches.get(2).unwrap().as_str(),
            matches.get(3).unwrap().as_str(),
        );
    }
    // Part 1
    let result = map.step_count("AAA", "ZZZ").unwrap();
    println!("The result for part 1 is {}", result);
    // Part 2
    let result = map.ghost_step_count('A', 'Z').unwrap();
    println!("The result for part 2 is {}", result);
}
