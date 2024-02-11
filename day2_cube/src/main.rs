use regex::Regex;
use std::str::FromStr;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use once_cell::sync::Lazy;

#[derive(Default)]
struct GameTally {
    games: Vec<Game>,
}

#[derive(Default, Debug)]
struct Game {
    id: u8,
    draws: Vec<ColorCount>,
}

#[derive(Default, Debug)]
struct ColorCount {
    red: u8,
    blue: u8,
    green: u8,
}

#[derive(Debug)]
struct ParseError {
    reason: String,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Game {
    fn parse(s: &str) -> Result<Self, ParseError> {
        let id = Self::extract_game_id(s)?;
        let draws = Self::extract_draws(s)?;

        Ok(Game { id, draws })
    }
    fn extract_game_id(s: &str) -> Result<u8, ParseError> {
        static regex: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Game (?P<game_id>\d+):").unwrap());
        let result = regex.captures(s).ok_or(ParseError {
            reason: "The regex didn't match Game Id :-/".to_string(),
        })?;
        let game_id = result.name("game_id").ok_or(ParseError {
            reason: "Can't extract the game id from the regex".to_string(),
        })?;
        game_id.as_str().parse().or(Err(ParseError {
            reason: "Can't convert game_id to u8".to_string(),
        }))
    }

    fn extract_draws(s: &str) -> Result<Vec<ColorCount>, ParseError> {
        let mut result = Vec::new();
        // Extract the color tallies
        static regex: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game \d+: (.*)").unwrap());
        let caps = regex.captures(s).ok_or(ParseError {
            reason: "The regex didn't catch the tallies :-/".to_string(),
        })?;
        let tallies = caps.get(1).unwrap().as_str();
        // parse each tally
        static regex2: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^:;]+").unwrap());
        for tally in regex2.find_iter(tallies).map(|m| m.as_str()) {
            result.push(tally.parse()?);
        }
        Ok(result)
    }
    fn is_possible(self: &Self, max_color: &ColorCount) -> bool {
        for draw in &self.draws {
            if draw.red > max_color.red {
                return false;
            }
            if draw.green > max_color.green {
                return false;
            }
            if draw.blue > max_color.blue {
                return false;
            }
        }
        true
    }

    fn minimum_set_of_cubes(self: &Self) -> ColorCount {
        let mut result = ColorCount::default();
        for draw in &self.draws {
            if draw.red > result.red { result.red = draw.red; }
            if draw.green > result.green { result.green = draw.green; }
            if draw.blue > result.blue { result.blue = draw.blue; }
        }
        result
    }

    fn minimum_power(self: &Self) -> u32 {
        self.minimum_set_of_cubes().power()
    }
}

impl ColorCount {
    fn power(self: &Self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

impl FromStr for ColorCount {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = ColorCount::default();
        static regex: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) (\w+)(?:, )?").unwrap());
        for (_, [count, color]) in regex.captures_iter(s).map(|c| c.extract()) {
            let count = count.parse().or(Err(ParseError {
                reason: "Can't convert color count to u8".to_string(),
            }))?;
            match color {
                "red" => result.red = count,
                "green" => result.green = count,
                "blue" => result.blue = count,
                _ => {
                    return Err(ParseError {
                        reason: "Unknown color".to_string(),
                    })
                }
            }
        }
        Ok(result)
    }
}

impl GameTally {
    fn push(self: &mut Self, game: Game) {
        self.games.push(game);
    }

    fn sum_possible_game_id(self: &Self, max_color: &ColorCount) -> u32 {
        let mut result = 0;
        for game in &self.games {
            if game.is_possible(max_color) {
                result += game.id as u32;
            }
        }
        result
    }

    fn sum_of_minimum_power(self: &Self) -> u32 {
        self.games.iter().map(|g| g.minimum_power()).sum()
    }
}

fn get_lines_from_files(filename: &str) -> impl Iterator<Item=String> {
    let file = fs::File::open(filename).unwrap();
    let file_buffer = BufReader::new(file);
    file_buffer.lines().map(|l| l.unwrap())
}

fn main() -> Result<(), ParseError> {
    let mut game_tally = GameTally::default();
    for line in get_lines_from_files("input.txt") {
        game_tally.push(line.parse()?);
    }
    let max_color = ColorCount{red: 12, green: 13, blue: 14};
    let result = game_tally.sum_possible_game_id(&max_color);
    println!("The sum of possible game id: {}", result);
    println!("The sum of minium powers: {}", game_tally.sum_of_minimum_power());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE: &str = "Game 1: 4 green, 7 blue; 2 blue, 4 red; 5 blue, 2 green, 2 red; 1 green, 3 red, 9 blue; 3 green, 9 blue; 7 green, 2 blue, 2 red";
    const LINE2: &str = "Game 2: 1 blue, 2 red; 1 green, 2 blue, 1 red; 1 red, 5 green; 3 red, 2 blue, 8 green; 3 blue, 2 red, 4 green; 2 blue, 4 green, 3 red";
    #[test]
    fn parse_game_id() -> Result<(), ParseError> {
        let game: Game = LINE.parse()?;
        assert_eq!(game.id, 1);
        Ok(())
    }

    #[test]
    fn parse_single_draw() -> Result<(), ParseError> {
        let string = "7 green, 2 blue";
        let count: ColorCount = string.parse()?;
        assert_eq!(count.red, 0);
        assert_eq!(count.green, 7);
        assert_eq!(count.blue, 2);
        Ok(())
    }

    #[test]
    fn parse_game_nb_draws() -> Result<(), ParseError> {
        let game: Game = LINE.parse()?;
        assert_eq!(game.draws.len(), 6);
        Ok(())
    }

    #[test]
    fn game_is_possible() -> Result<(), ParseError> {
        let max = ColorCount { red: 25, green: 25, blue: 25};
        let game: Game = LINE.parse()?;
        assert!(game.is_possible(&max));
        Ok(())
    }

    #[test]
    fn game_is_not_possible() -> Result<(), ParseError> {
        let max = ColorCount { red: 25, green: 3, blue: 25};
        let game: Game = LINE.parse()?;
        assert!(!game.is_possible(&max));
        Ok(())
    }

    #[test]
    fn game_tally_push_game() -> Result<(), ParseError> {
        let mut tally = GameTally::default();
        tally.push(LINE.parse()?);
        tally.push(LINE2.parse()?);
        assert_eq!(tally.games.len(), 2);
        Ok(())
    }

    #[test]
    fn game_tally_add_possible_game_id() -> Result<(), ParseError> {
        let max = ColorCount { red: 25, green: 25, blue: 25};
        let mut tally = GameTally::default();
        tally.push(LINE.parse()?);
        tally.push(LINE2.parse()?);
        assert_eq!(tally.sum_possible_game_id(&max), 3);
        Ok(())
    }

    #[test]
    fn game_minimum_set_of_cubes() -> Result<(), ParseError> {
        let game: Game = LINE.parse()?;
        let min_set = game.minimum_set_of_cubes();
        assert_eq!(min_set.red, 4);
        assert_eq!(min_set.blue, 9);
        assert_eq!(min_set.green, 7);
        Ok(())
    }

    #[test]
    fn game_minimum_power() -> Result<(), ParseError> {
        let game: Game = LINE.parse()?;
        assert_eq!(game.minimum_power(), 4*9*7);
        Ok(())
    }
}
