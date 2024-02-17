use super::card;
use crate::camel_joker::card::Card;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [card::Card; 5],
}

#[derive(Debug)]
pub struct HandParseError;

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand {
            cards: s
                .chars()
                .map(card::Card::from_char)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let force_self = self.force();
        let force_other = other.force();
        if force_self != force_other {
            return Some(force_self.cmp(&force_other));
        }
        if self.cards == other.cards {
            return Some(Ordering::Equal);
        }
        for i in 0..5 {
            let result = self.cards[i].partial_cmp(&other.cards[i]).unwrap();
            if result != Ordering::Equal {
                return Some(result);
            }
        }
        panic!("Bug on Hand::partial_cmp")
    }
}

impl Hand {
    // Force of the hand (1: high card, 2: one pair, 3: two pairs, ...)
    fn force(&self) -> u8 {
        let mut count = HashMap::new();
        for card in &self.cards {
            *count.entry(card).or_insert(0) += 1;
        }
        // deal with jokers
        let joker_count = count.remove(&Card::Joker).unwrap_or(0);
        if joker_count == 5 {
            return 7;
        } // Five of a kind
        let mut score: Vec<_> = count.values().collect();
        score.sort();
        score.reverse();
        match score[0] + joker_count {
            5 => 7, // Five of a kind
            4 => 6, // Four of a kind
            3 => {
                match score[1] {
                    2 => 5, // Full house (3+2)
                    _ => 4, // Three of a kind
                }
            }
            2 => {
                match score[1] {
                    2 => 3, // two pairs
                    _ => 2, // one pair
                }
            }
            _ => 1, // high card
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let expected = Hand {
            cards: [
                card::Card::King,
                card::Card::Queen,
                card::Card::Joker,
                card::Card::Ace,
                card::Card::Number(card::CardNumber::new(5)),
            ],
        };
        assert_eq!("KQJA5".parse::<Hand>().unwrap(), expected);
    }

    #[test]
    fn force() {
        assert_eq!("55555".parse::<Hand>().unwrap().force(), 7);
        assert_eq!("JJJJJ".parse::<Hand>().unwrap().force(), 7);
        assert_eq!("KKKK5".parse::<Hand>().unwrap().force(), 6);
        assert_eq!("K22KK".parse::<Hand>().unwrap().force(), 5);
        assert_eq!("KK52K".parse::<Hand>().unwrap().force(), 4);
        assert_eq!("KQQ7K".parse::<Hand>().unwrap().force(), 3);
        assert_eq!("77T88".parse::<Hand>().unwrap().force(), 3);
        assert_eq!("44KQJ".parse::<Hand>().unwrap().force(), 4);
        assert_eq!("87654".parse::<Hand>().unwrap().force(), 1);
        assert_eq!("8765J".parse::<Hand>().unwrap().force(), 2);
    }

    #[test]
    fn cmp_different_force() {
        assert_eq!(
            "55555".parse::<Hand>().unwrap() < "55455".parse::<Hand>().unwrap(),
            false
        );
        assert_eq!(
            "55522".parse::<Hand>().unwrap() < "55555".parse::<Hand>().unwrap(),
            true
        );
        assert_eq!(
            "55555".parse::<Hand>().unwrap() < "55555".parse::<Hand>().unwrap(),
            false
        );
        assert_eq!(
            "55KKQ".parse::<Hand>().unwrap() > "5546Q".parse::<Hand>().unwrap(),
            true
        );
        assert_eq!(
            "88QTK".parse::<Hand>().unwrap() < "65432".parse::<Hand>().unwrap(),
            false
        );
    }

    #[test]
    fn cmp_same_force() {
        assert_eq!(
            "55555".parse::<Hand>().unwrap() < "44444".parse::<Hand>().unwrap(),
            false
        );
        assert_eq!(
            "55522".parse::<Hand>().unwrap() < "55533".parse::<Hand>().unwrap(),
            true
        );
        assert_eq!(
            "55Q99".parse::<Hand>().unwrap() < "77T88".parse::<Hand>().unwrap(),
            true
        );
    }
}
