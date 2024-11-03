use crate::day10_pipe_maze::map::{Map, Sector};
use crate::day10_pipe_maze::pipe_type::PipeType;
use crate::day10_pipe_maze::position::Position;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Copy, Clone)]
enum RayPosition {
    North,
    Center,
    South,
}

pub struct Path<'a> {
    map: &'a Map,
    path: HashSet<Sector>,
}

impl<'a> Path<'a> {
    pub fn new(map: &'a Map) -> Self {
        Path {
            map,
            path: Self::find_path(map),
        }
    }

    pub fn steps_to_be_farthest(&self) -> usize {
        self.path.len() / 2
    }

    pub fn get_sectors_inside_paths(&self) -> Vec<Sector> {
        let mut inside_sectors = Vec::new();
        for y in 0..self.map.get_height() as isize {
            let mut inside: bool = false;
            let mut ray_position = RayPosition::Center;
            for x in 0..self.map.get_width() as isize {
                let sector_under_analysis = self
                    .map
                    .get_sector(Position(x, y))
                    .expect("BUG: Trying to get out of bound sector.");
                // the sector on a barrier
                if self.path.contains(sector_under_analysis) {
                    // TODO Probably buggy what about corners
                    match sector_under_analysis.pipe_type() {
                        PipeType::NorthSouth => {
                            inside = !inside;
                            ray_position = RayPosition::Center;
                        }
                        PipeType::SouthWest if ray_position == RayPosition::South => {
                            inside = !inside;
                            ray_position = RayPosition::Center;
                        }
                        PipeType::NorthWest if ray_position == RayPosition::North => {
                            inside = !inside;
                            ray_position = RayPosition::Center;
                        }
                        PipeType::NorthEast => ray_position = RayPosition::South,
                        PipeType::SouthEast => ray_position = RayPosition::North,
                        _ => (),
                    }
                    continue;
                }
                // odd => within the path
                if inside {
                    inside_sectors.push(*sector_under_analysis);
                }
            }
        }
        inside_sectors
    }

    fn find_path(map: &Map) -> HashSet<Sector> {
        let mut path = HashSet::new();
        let starting_point = map.get_starting_point();
        let neighbors = map.get_next_sectors(starting_point);

        if neighbors.len() != 2 {
            panic!(
                "Starting point must have exactly 2 neighbors, found {}",
                neighbors.len()
            );
        }

        path.insert(*starting_point);
        let mut cursor = neighbors[0];
        path.insert(*cursor);

        if neighbors.len() != 2 {
            panic!(
                "All points in path must have exactly 2 neighbors, found {} at {:?}",
                neighbors.len(),
                cursor
            );
        }
        loop {
            let neighbors = map.get_next_sectors(cursor);
            match (path.contains(neighbors[0]), path.contains(neighbors[1])) {
                (false, true) => cursor = neighbors[0],
                (true, false) => cursor = neighbors[1],
                (true, true) => break,
                (false, false) => panic!(),
            }
            path.insert(*cursor);
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_loop() {
        let data = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        let map: Map = data.parse().unwrap();
        let path = Path::new(&map);
        assert_eq!(path.steps_to_be_farthest(), 4);
    }

    #[test]
    fn get_inside_sectors_example_1() {
        let data = include_str!("resources/test_p2_1.txt");
        let map: Map = data.parse().unwrap();
        let path = Path::new(&map);
        let inside_sectors = path.get_sectors_inside_paths();
        assert_eq!(
            inside_sectors.len(),
            4,
            "There should be exactly 4 sector inside path.\nSector detected: {:?}",
            inside_sectors
        );
    }

    #[test]
    fn get_inside_sectors_example_2() {
        let data = include_str!("resources/test_p2_2.txt");
        let map: Map = data.parse().unwrap();
        let path = Path::new(&map);
        let inside_sectors = path.get_sectors_inside_paths();
        assert_eq!(
            inside_sectors.len(),
            4,
            "There should be exactly 4 sector inside path.\nSector detected: {:?}",
            inside_sectors
        );
    }

    #[test]
    fn get_inside_sectors_example_3() {
        let data = include_str!("resources/test_p2_3.txt");
        let map: Map = data.parse().unwrap();
        let path = Path::new(&map);
        let inside_sectors = path.get_sectors_inside_paths();
        assert_eq!(
            inside_sectors.len(),
            8,
            "There should be exactly 8 sector inside path.\nSector detected: {:?}",
            inside_sectors
        );
    }

    #[test]
    fn get_inside_sectors_example_4() {
        let data = include_str!("resources/test_p2_4.txt");
        let map: Map = data.parse().unwrap();
        let path = Path::new(&map);
        let inside_sectors = path.get_sectors_inside_paths();
        assert_eq!(
            inside_sectors.len(),
            10,
            "There should be exactly 10 sector inside path.\nSector detected: {:?}",
            inside_sectors
        );
    }
}
