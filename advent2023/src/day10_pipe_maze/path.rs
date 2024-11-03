use crate::day10_pipe_maze::map::{Map, Sector};
use std::collections::HashSet;

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
}
