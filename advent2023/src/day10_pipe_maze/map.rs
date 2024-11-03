use super::pipe_type::{InvalidPositionError, PipeType};
use super::position::{Displacement, Position};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub struct Sector {
    position: Position,
    pipe_type: PipeType,
}

#[derive(Debug)]
pub enum ParseMapError {
    EmptyString,
    LineLength(usize),
    PositionError(InvalidPositionError),
    MultipleStartingPositions(Position),
    NoStartingPosition,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MapError {
    OutOfBounds,
}

#[derive(Debug)]
pub struct Map {
    // TODO why not a HashMap Position => sector instead?
    positions: Box<[Box<[Sector]>]>,
    starting_position: Position,
}

// TODO Should that be a different object like a Factory or something?
impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(ParseMapError::EmptyString);
        }
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        let mut sector_rows: Vec<Box<[Sector]>> = Vec::with_capacity(height);

        let mut starting_position: Option<Position> = None;
        for (y, line) in s.lines().enumerate() {
            if line.chars().count() != width {
                return Err(ParseMapError::LineLength(y));
            }
            let mut sector_row: Vec<Sector> = Vec::with_capacity(width);
            for (x, p) in line.chars().enumerate() {
                let pipe_type =
                    PipeType::try_from(p).map_err(|e| ParseMapError::PositionError(e))?;
                let position = Position(x as isize, (height - 1 - y) as isize);
                let sector = Sector {
                    position,
                    pipe_type,
                };
                sector_row.push(sector);
                if pipe_type == PipeType::StartingPosition {
                    match starting_position {
                        None => starting_position = Some(position),
                        Some(coords) => {
                            return Err(ParseMapError::MultipleStartingPositions(coords));
                        }
                    }
                }
            }
            sector_rows.push(sector_row.into_boxed_slice());
        }
        let mut result = match starting_position {
            Some(start_coords) => Map {
                positions: sector_rows.into_boxed_slice(),
                starting_position: start_coords,
            },
            None => return Err(ParseMapError::NoStartingPosition),
        };
        result.infer_starting_point_type();
        Ok(result)
    }
}

impl Map {
    fn infer_starting_point_type(&mut self) {
        let starting_point_position = self.starting_position;
        let starting_point_sector = self.get_sector(starting_point_position).unwrap();
        let tests: HashMap<&str, Displacement> = HashMap::from([
            ("east", Displacement(1, 0)),
            ("west", Displacement(-1, 0)),
            ("north", Displacement(0, 1)),
            ("south", Displacement(0, -1)),
        ]);
        let mut results: HashMap<&str, bool> = HashMap::from([
            ("east", false),
            ("west", false),
            ("north", false),
            ("south", false),
        ]);
        for (direction, dis) in tests {
            let cobaye_sector = self.get_sector(starting_point_position + dis);
            match cobaye_sector {
                Ok(sector) => {
                    if self
                        .get_next_sectors(sector)
                        .contains(&starting_point_sector)
                    {
                        results.entry(direction).and_modify(|b| *b = true);
                    }
                }
                Err(_) => continue,
            }
        }
        let pipe_type: PipeType;
        // NorthSouth
        if results["north"] && results["south"] {
            pipe_type = PipeType::NorthSouth;
        } else if results["east"] && results["west"] {
            pipe_type = PipeType::EastWest;
        } else if results["north"] && results["east"] {
            pipe_type = PipeType::NorthEast;
        } else if results["north"] && results["west"] {
            pipe_type = PipeType::NorthWest;
        } else if results["south"] && results["west"] {
            pipe_type = PipeType::SouthWest;
        } else if results["south"] && results["east"] {
            pipe_type = PipeType::SouthEast;
        } else {
            panic!("Starting point not connected to two neighbors")
        }
        self.get_sector_mut(starting_point_position)
            .unwrap()
            .pipe_type = pipe_type;
    }

    pub fn get_starting_point(&self) -> &Sector {
        self.get_sector(self.starting_position).unwrap()
    }

    // return two indexes that can be used in positions[a][b]
    fn position_to_index(&self, position: &Position) -> (usize, usize) {
        (
            self.get_height() - 1 - position.1 as usize,
            position.0 as usize,
        )
    }

    fn is_position_valid(&self, position: &Position) -> bool {
        !(position.0 < 0
            || position.1 < 0
            || position.0 >= self.get_width() as isize
            || position.1 >= self.get_height() as isize)
    }
    pub fn get_sector(&self, position: Position) -> Result<&Sector, MapError> {
        if !self.is_position_valid(&position) {
            return Err(MapError::OutOfBounds);
        }
        let (a, b) = self.position_to_index(&position);
        Ok(&self.positions[a][b])
    }

    pub fn get_sector_mut(&mut self, position: Position) -> Result<&mut Sector, MapError> {
        if !self.is_position_valid(&position) {
            return Err(MapError::OutOfBounds);
        }
        let (a, b) = self.position_to_index(&position);
        Ok(&mut self.positions[a][b])
    }

    pub fn get_width(&self) -> usize {
        self.positions.first().unwrap().len()
    }

    pub fn get_height(&self) -> usize {
        self.positions.len()
    }

    pub fn get_next_sectors(&self, sector: &Sector) -> Vec<&Sector> {
        let mut result: Vec<&Sector> = Vec::new();
        for displacement in sector.pipe_type.get_valid_connections() {
            match self.get_sector(sector.position + displacement) {
                Ok(sector) => result.push(sector),
                Err(_) => continue,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_from_str() {
        let char_map = "JJJ\n|||\nFFF\n-S-";
        let map: Map = char_map.parse().unwrap();
        assert_eq!(map.positions[0][1].pipe_type, PipeType::NorthWest);
    }

    #[test]
    fn map_from_empty_str() {
        let map = "".parse::<Map>();
        assert!(map.is_err());
        match map {
            Err(ParseMapError::EmptyString) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn map_from_str_invalid_str() {
        let char_map = "JJJ\n|||\nFF\nSSS";
        let map = char_map.parse::<Map>();
        assert!(map.is_err());
        match map {
            Err(ParseMapError::LineLength(2)) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn map_from_str_line_invalid_position() {
        let char_map = "JJJ\n|||\nF!F\nSSS";
        let map = char_map.parse::<Map>();
        assert!(map.is_err());
        match map {
            Err(ParseMapError::PositionError(InvalidPositionError('!'))) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn map_from_str_no_starting_point() {
        let char_map = "JJJ\n|||\nFFF\n---";
        let map = char_map.parse::<Map>();
        assert!(map.is_err());
        match map {
            Err(ParseMapError::NoStartingPosition) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn map_from_str_multiple_starting_point() {
        let char_map = "JSJ\n|||\nFFF\n-S-";
        let map = char_map.parse::<Map>();
        assert!(map.is_err());
        println!("{:?}", map);
        match map {
            Err(ParseMapError::MultipleStartingPositions(Position(1, 3))) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_get_next_coordinates() {
        let char_map = "F-7\n|.|\n|.|\nLSJ";
        let map = char_map.parse::<Map>().unwrap();
        let next_sectors = map.get_next_sectors(map.get_sector(Position(0, 0)).unwrap());
        assert_eq!(next_sectors.len(), 2);
        assert!(next_sectors.contains(&map.get_sector(Position(0, 1)).unwrap()));
        assert!(next_sectors.contains(&map.get_sector(Position(1, 0)).unwrap()));
    }

    #[test]
    fn test_get_next_coordinates_ground() {
        let char_map = "F-7\n|.|\n|.|\nLSJ";
        let map = char_map.parse::<Map>().unwrap();
        assert!(map
            .get_next_sectors(map.get_sector(Position(1, 1)).unwrap())
            .is_empty());
    }

    #[test]
    fn test_get_next_coordinates_starting_position() {
        let char_map = "F-7\n|.|\n|.|\nLSJ";
        let map = char_map.parse::<Map>().unwrap();
        let next_sectors = map.get_next_sectors(map.get_sector(Position(1, 0)).unwrap());
        assert_eq!(next_sectors.len(), 2);
        assert!(next_sectors.contains(&map.get_sector(Position(0, 0)).unwrap()));
        assert!(next_sectors.contains(&map.get_sector(Position(2, 0)).unwrap()));
    }

    #[test]
    fn test_get_sector() {
        let char_map = "F-7\n|.|\n|.|\nLSJ";
        let map = char_map.parse::<Map>().unwrap();
        assert_eq!(
            map.get_sector(Position(0, 0)).unwrap(),
            &Sector {
                position: Position(0, 0),
                pipe_type: PipeType::NorthEast
            }
        );
    }

    #[test]
    fn test_get_sector_out_of_bounds() {
        let char_map = "F-7\n|.|\n|.|\nLSJ";
        let map = char_map.parse::<Map>().unwrap();
        assert_eq!(map.get_sector(Position(1, 72)), Err(MapError::OutOfBounds));
    }

    #[test]
    fn test_infer_starting_point_pipe_type() {
        let char_map = "F-7\n|.|\n|.|\nLSJ";
        let map = char_map.parse::<Map>().unwrap();
        assert_eq!(
            map.get_starting_point(),
            &Sector {
                position: Position(1, 0),
                pipe_type: PipeType::EastWest
            }
        );
    }
}
