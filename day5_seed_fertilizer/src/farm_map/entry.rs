use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct MapEntry {
    pub source_start: u64,
    pub target_start: u64,
    pub range_size: u64,
}

#[derive(Debug)]
pub enum MapEntryError {
    ParseError(String),
}

impl Display for MapEntryError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Oh no => {:?}", self)
    }
}

impl Error for MapEntryError {}

impl MapEntry {
    pub fn convert(&self, source: u64) -> u64 {
        if source < self.source_start {
            panic!("BUG: source must be >= to source_start");
        }
        if source < self.source_start + self.range_size {
            self.target_start + source - self.source_start
        } else {
            source
        }
    }
}

impl FromStr for MapEntry {
    type Err = MapEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number_strs: Vec<&str> = s.split(' ').collect();
        if number_strs.len() != 3 {
            return Err(MapEntryError::ParseError(
                "the line to parse must have exactly 3 fields".to_string(),
            ));
        }
        let source_start = number_strs[1].parse().map_err(|err| {
            MapEntryError::ParseError(format!("error parsing source_start: {}", err))
        })?;
        let target_start = number_strs[0].parse().map_err(|err| {
            MapEntryError::ParseError(format!("error parsing target_start: {}", err))
        })?;
        let range_size = number_strs[2].parse().map_err(|err| {
            MapEntryError::ParseError(format!("error parsing range_size: {}", err))
        })?;
        Ok(MapEntry {
            source_start,
            target_start,
            range_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTRY: MapEntry = MapEntry {
        source_start: 42,
        target_start: 100,
        range_size: 20,
    };
    #[test]
    #[should_panic]
    fn map_entry_convert_source_too_small() {
        ENTRY.convert(40);
    }

    #[test]
    fn map_entry_convert_in_range() {
        assert_eq!(ENTRY.convert(52), 110);
    }

    #[test]
    fn map_entry_convert_at_range_start() {
        assert_eq!(ENTRY.convert(52), 110);
    }

    #[test]
    fn map_entry_convert_at_range_end() {
        assert_eq!(ENTRY.convert(61), 119);
        assert_eq!(ENTRY.convert(62), 62);
    }

    #[test]
    fn map_entry_convert_out_of_range() {
        assert_eq!(ENTRY.convert(125), 125);
    }

    #[test]
    fn map_entry_parse() -> Result<(), MapEntryError> {
        let entry: MapEntry = "1456 7896 25".parse()?;
        let expected = MapEntry {
            source_start: 7896,
            target_start: 1456,
            range_size: 25,
        };
        assert_eq!(entry, expected);
        Ok(())
    }
}
