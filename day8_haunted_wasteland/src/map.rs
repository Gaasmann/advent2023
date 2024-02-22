use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Result<Direction, ParseError> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(ParseError {}),
        }
    }
    pub fn build_vec(s: &str) -> Result<Vec<Direction>, ParseError> {
        s.chars().map(Self::from_char).collect()
    }
}

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug)]
struct Cell {
    left: String,
    right: String,
}

#[derive(Debug)]
pub struct Map {
    directions: Vec<Direction>,
    tree: HashMap<String, Cell>,
}

impl Map {
    pub fn new(directions: Vec<Direction>) -> Map {
        Map {
            directions,
            tree: HashMap::new(),
        }
    }
    pub fn add_cell(&mut self, source: &str, left: &str, right: &str) {
        self.tree.insert(
            source.to_string(),
            Cell {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }

    pub fn next_step(&self, cell_name: &str, direction: &Direction) -> &str {
        match direction {
            Direction::Left => self.tree[cell_name].left.as_str(),
            Direction::Right => self.tree[cell_name].right.as_str(),
        }
    }
    pub fn step_count(&self, source: &str, dest: &str) -> Result<usize, ()> {
        let mut current = source;
        for (count, direction) in self.directions.iter().cycle().enumerate() {
            current = self.next_step(current, direction);

            if current == dest {
                return Ok(count + 1);
            }
        }
        Err(())
    }
    pub fn ghost_step_count(
        &self,
        source_end_letter: char,
        dest_end_letter: char,
    ) -> Result<usize, ()> {
        // find all starting nodes
        let nodes: Vec<&str> = self
            .tree
            .keys()
            .filter(|&node| node.chars().last().unwrap() == source_end_letter)
            .map(|s| s.as_str())
            .collect();
        let step_counts: Vec<usize> = nodes
            .iter()
            .map(|nodes| self.ghost_step_one_path(nodes, dest_end_letter).unwrap())
            .collect();
        // lcm(a,b,c) = lcm(a, lcm(b,c))
        let result = step_counts
            .iter()
            .copied()
            .reduce(lcm)
            .unwrap();
        Ok(result)
    }

    // TODO make up your mind between cell and node
    fn ghost_step_one_path(&self, source_cell: &str, dest_end_letter: char) -> Result<usize, ()> {
        let mut current = source_cell;
        for (count, direction) in self.directions.iter().cycle().enumerate() {
            current = self.next_step(current, direction);
            if current.chars().last().unwrap() == dest_end_letter {
                return Ok(count + 1);
            }
        }
        Err(())
    }
}
