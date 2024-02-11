use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Card {
    card_number: usize,
    winning_numbers: HashSet<u8>,
    played_numbers: HashSet<u8>,
}

#[derive(Debug)]
enum ParseError {
    FormatError,
    ParseCardNumber,
}

impl Card {
    fn parse(s: &str) -> Result<Self, ParseError> {
        // parse card number
        let (card, rest) = s.split_once(':').ok_or(ParseError::FormatError)?;
        let card_number: usize = Self::parse_card(card)?;
        // parse numbers
        let (winning_str, played_str) = rest
            .split_once('|')
            .ok_or(ParseError::FormatError)?;
        let winning_numbers = Self::parse_numbers(winning_str)?;
        let played_numbers = Self::parse_numbers(played_str)?;
        Ok(Card {
            card_number,
            winning_numbers,
            played_numbers,
        })
    }
    fn parse_card(s: &str) -> Result<usize, ParseError> {
        static REGEX_CARD: Lazy<Regex> = Lazy::new(|| Regex::new(r"Card\s+(\d+)").unwrap());
        let card_number: usize = REGEX_CARD
            .captures(s)
            .ok_or(ParseError::FormatError)?
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| ParseError::ParseCardNumber)?;
        Ok(card_number)
    }
    fn parse_numbers(s: &str) -> Result<HashSet<u8>, ParseError> {
        static REGEX_NUMBERS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
        let numbers: HashSet<u8> = REGEX_NUMBERS
            .find_iter(s)
            .map(|m| m.as_str())
            .map(|s| s.parse::<u8>().unwrap()) // TODO How to handle errors in a map correctly
            .collect();
        Ok(numbers)
    }
    fn count_matches(&self) -> usize {
        self.winning_numbers
            .intersection(&self.played_numbers)
            .count()
    }

    fn score(&self) -> usize {
        match self.count_matches() {
            0 => 0,
            count => 2usize.pow((count - 1).try_into().unwrap()),
        }
    }
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[derive(Debug)]
enum DocumentError {
    OpenFile,
}

fn read_card(filename: &str) -> Result<impl Iterator<Item = String>, DocumentError> {
    let file = File::open(filename).map_err(|_| DocumentError::OpenFile)?;
    let buf = BufReader::new(file);
    Ok(buf.lines().map(|r| r.unwrap())) // TODO How to deal with error in maps
}

fn main() {
    let original_deck: Vec<Card> = read_card("input.txt")
        .unwrap()
        .map(|line| line.parse::<Card>().unwrap()) // TODO How to deal with error in maps
        .collect();
    // Part 1
    let result: usize = original_deck.iter().map(|card| card.score()).sum();
    println!("The total score for part 1 is: {}", result);
    // Part 2
    let mut card_count: Vec<usize> = vec![1; original_deck.len()];
    for current_card in &original_deck {
        let nb_copies = card_count[current_card.card_number - 1];
        let count_matches = current_card.count_matches();
        for i in current_card.card_number + 1..current_card.card_number + count_matches + 1 {
            if i <= original_deck.len() {
                card_count[i - 1] += nb_copies;
            } else {
                break;
            }
        }
    }

    println!(
        "The total number of scratchcards for part 2 is: {}",
        card_count.iter().sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LINE: &str = "Card 201: 67 83  3 45 26 56  4 57 75 19 | 16  1 68 43 27 82 55 71 37 59 33 93 13  8 25 12 15 44 11 48 41 17 47 36 35";
    #[test]
    fn test_parsing() {
        let card: Card = TEST_LINE.parse().unwrap();
        assert_eq!(card.card_number, 201);
        assert_eq!(
            card.winning_numbers,
            HashSet::from([67u8, 83, 3, 45, 26, 56, 4, 57, 75, 19])
        );
        assert_eq!(
            card.played_numbers,
            HashSet::from([
                16u8, 1, 68, 43, 27, 82, 55, 71, 37, 59, 33, 93, 13, 8, 25, 12, 15, 44, 11, 48, 41,
                17, 47, 36, 35
            ])
        );
    }
    const TEST_LINE_SCORE: &str = "Card 201: 67 83  3 45 26 56  4 57 75 19 | 16  1 68 43 27 82 57 71 37 59 33 93 13  8 25 12 3 44 11 4 41 17 47 36 35";
    #[test]
    fn test_score() {
        let card: Card = TEST_LINE.parse().unwrap();
        assert_eq!(card.score(), 0);
        let card: Card = TEST_LINE_SCORE.parse().unwrap();
        assert_eq!(card.score(), 4);
    }
}
