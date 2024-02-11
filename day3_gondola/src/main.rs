use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Document {
    width: usize,
    data: Vec<char>,
}

impl Document {
    fn from_file(filename: &str) -> Result<Self, DocumentError> {
        let file = File::open(filename).map_err(|_| DocumentError::OpenFile)?;
        let buf = BufReader::new(file);
        let mut iter = buf.lines();
        let string_buffer = iter.next().ok_or(DocumentError::EmptyFile)?.unwrap();
        let mut data: Vec<char> = string_buffer.chars().collect();
        let width = data.len();
        for line in iter {
            data.extend(line.unwrap().chars());
        }
        Ok(Document { width, data })
    }

    fn find_next_number(&self, cur_pos: usize) -> Option<(usize, usize, usize)> {
        // begin, end, number
        let mut cursor = cur_pos;
        let mut begin = None;
        let mut end = None;
        while let Some(value) = self.data.get(cursor) {
            if begin.is_none() && value.is_ascii_digit() {
                begin = Some(cursor);
            } else if begin.is_some() && !value.is_ascii_digit() {
                end = Some(cursor);
                break;
            }
            cursor += 1;
        }
        if let Some(end) = end {
            let begin = begin.unwrap();
            Some((
                begin,
                end,
                (self.data)[begin..end]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            ))
        } else {
            None
        }
    }

    fn is_number_valid(&self, idx_start: usize, idx_end: usize) -> bool {
        // Check the upper line
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^0-9.]").unwrap());
        let line_to_check: [isize; 3] = [-1, 0, 1];
        for i in line_to_check {
            let start_bound = idx_start as isize - 1 + i * self.width as isize;
            let end_bound = idx_end as isize + 1 + i * self.width as isize;
            if start_bound < 0 {
                continue;
            }
            if end_bound as usize >= self.data.len() {
                continue;
            }
            let to_check = &self.data[start_bound as usize..end_bound as usize];
            if REGEX.is_match(to_check.iter().collect::<String>().as_str()) {
                return true;
            }
        }
        false
    }

    fn find_and_add_gear_ratios(&self) -> usize {
        let mut gear_ratios = Vec::new();
        // Map number positions
        let mut number_position = HashMap::new();
        let mut cursor = 0usize;
        while let Some((begin, end, number)) = self.find_next_number(cursor) {
            for i in begin..end {
                number_position.insert(i, (begin, end, number));
            }
            cursor = end;
        }
        // find position of all stars
        let star_positions: Vec<usize> = self
            .data
            .iter()
            .enumerate()
            .filter(|(_, chr)| **chr == '*')
            .map(|(idx, _)| idx)
            .collect();
        println!("{:#?}", star_positions);
        // find numbers
        for star_pos in star_positions {
            let pos_to_check: [isize; 8] = [
                star_pos as isize - self.width as isize - 1,
                star_pos as isize - self.width as isize,
                star_pos as isize + 1 - self.width as isize,
                star_pos as isize - 1,
                star_pos as isize + 1,
                star_pos as isize + self.width as isize - 1,
                star_pos as isize + self.width as isize,
                star_pos as isize + 1 + self.width as isize,
            ];
            let mut number_set = HashSet::new();
            for pos in pos_to_check {
                if pos < 0 || pos >= self.data.len() as isize {
                    continue;
                }
                if let Some(number) = number_position.get(&(pos as usize)) {
                    number_set.insert(number);
                }
            }
            if number_set.len() == 2 {
                gear_ratios.push(number_set.iter().map(|(_, _, number)| number).product());
            }
        }
        gear_ratios.iter().sum()
    }
}

#[derive(Debug)]
enum DocumentError {
    OpenFile,
    EmptyFile,
}

fn main() {
    let doc = Document::from_file("input.txt").unwrap();
    let mut cursor = 0usize;
    let mut result: usize = 0;
    while let Some((begin, end, number)) = doc.find_next_number(cursor) {
        println!("{}:{}=>{}", begin, end, number);
        if doc.is_number_valid(begin, end) {
            result += number;
        }
        cursor = end;
    }
    println!("The result is: {result}");
    let result = doc.find_and_add_gear_ratios();
    println!("The gear result is: {result}");
}
