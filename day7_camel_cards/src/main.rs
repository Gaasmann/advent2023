mod camel;
mod camel_joker;

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn parse_file<H>(filename: &str) -> impl Iterator<Item = (H, u32)>
where
    H: FromStr,
    <H as FromStr>::Err: Debug,
{
    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand.to_owned(), bid.to_owned())
        })
        .map(|(hand, bid)| (hand.parse().unwrap(), bid.parse().unwrap()))
}

fn process_hands<H>(filename: &str) -> u32
where
    H: FromStr + PartialOrd,
    <H as FromStr>::Err: Debug,
{
    let mut hands: Vec<_> = parse_file::<H>(filename).collect();
    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let result: u32 = hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) as u32 * bid)
        .sum();
    result
}
fn main() {
    // part 1
    let result = process_hands::<camel::Hand>("input.txt");
    println!("The result for part 1 is {}", result);
    // part 2
    let result = process_hands::<camel_joker::Hand>("input.txt");
    println!("The result for part 2 is {}", result);
}
