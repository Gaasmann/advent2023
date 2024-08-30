use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Sequence {
    data: Vec<i32>,
}

#[derive(Debug, PartialEq)]
pub enum SequenceError {
    EmptyInput,
    ParseError(ParseIntError),
}

impl FromStr for Sequence {
    type Err = SequenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(SequenceError::EmptyInput);
        }

        let data: Result<Vec<i32>, ParseIntError> = s.split_whitespace().map(str::parse).collect();

        match data {
            Ok(vec) => Ok(Sequence { data: vec }),
            Err(e) => Err(SequenceError::ParseError(e)),
        }
    }
}

impl Sequence {
    fn generate_subsequences(&self) -> Vec<Vec<i32>> {
        let mut subsequences: Vec<Vec<i32>> = vec![self.data.clone()];
        while !subsequences.last().unwrap().iter().all(|v| *v == 0) {
            let previous_subsequence = subsequences.last().unwrap();
            let subsequence = (1..previous_subsequence.len())
                .map(|index| previous_subsequence[index] - previous_subsequence[index - 1])
                .collect();
            subsequences.push(subsequence);
        }
        subsequences
    }
    pub fn predict_next(&self) -> i32 {
        let subsequences: Vec<Vec<i32>> = self.generate_subsequences();
        let mut to_add = 0i32;
        for sequence in subsequences.iter().rev().skip(1) {
            to_add = to_add + sequence.last().unwrap();
        }
        to_add
    }

    pub fn extrapolate_previous(&self) -> i32 {
        let subsequences: Vec<Vec<i32>> = self.generate_subsequences();
        let mut to_sub = 0i32;
        for sequence in subsequences.iter().rev().skip(1) {
            to_sub = sequence.first().unwrap() - to_sub;
        }
        to_sub
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sequence_from_string() {
        let data = "12 42 -7 5 0";
        let expected_data = vec![12, 42, -7, 5, 0];
        assert_eq!(Sequence::from_str(data).unwrap().data, expected_data);
    }

    #[test]
    fn create_sequence_from_empty_string() {
        let result = Sequence::from_str("");
        assert!(result.is_err());
        assert_eq!(result, Err(SequenceError::EmptyInput));
    }

    #[test]
    fn create_sequence_from_garbage_string() {
        let result = Sequence::from_str("10 arthur -5");
        assert!(result.is_err());
        match result {
            Err(SequenceError::ParseError(_)) => {}
            _ => panic!("Expected ParseError, got {:?}", result),
        }
    }

    #[test]
    fn test_predict_next() {
        let sequence = Sequence::from_str("1 3 6 10 15 21").unwrap();
        assert_eq!(sequence.predict_next(), 28)
    }

    #[test]
    fn test_extrapolate_previous() {
        let sequence = Sequence::from_str("10 13 16 21 30 45").unwrap();
        assert_eq!(sequence.extrapolate_previous(), 5)
    }
}
