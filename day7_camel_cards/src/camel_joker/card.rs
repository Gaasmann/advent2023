use std::cmp::Ordering;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
pub struct CardNumber(u8);

impl CardNumber {
    pub fn new(value: u8) -> CardNumber {
        if (2..=10).contains(&value) {
            return CardNumber(value);
        }
        panic!("Invalid card number entry!!")
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Card {
    Ace,
    King,
    Queen,
    Joker,
    Number(CardNumber),
}

impl Card {
    pub fn from_char(chr: char) -> Self {
        if chr.is_ascii_digit() {
            Self::Number(CardNumber::new(chr.to_digit(10).unwrap() as u8))
        } else {
            match chr {
                'A' => Self::Ace,
                'K' => Self::King,
                'Q' => Self::Queen,
                'J' => Self::Joker,
                'T' => Self::Number(CardNumber::new(10)),
                _ => panic!("Wrong character for creating a card."),
            }
        }
    }
    fn numeric_value(&self) -> u8 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Joker => 1,
            Card::Number(value) => value.0,
        }
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.numeric_value().cmp(&other.numeric_value()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp_numbers() {
        assert_eq!(
            Card::Number(CardNumber::new(3)) < Card::Number(CardNumber::new(5)),
            true
        );
        assert_eq!(
            Card::Number(CardNumber::new(3)) > Card::Number(CardNumber::new(5)),
            false
        );
        assert_eq!(
            Card::Number(CardNumber::new(8)) == Card::Number(CardNumber::new(8)),
            true
        );
        assert_eq!(
            Card::Number(CardNumber::new(8)) == Card::Number(CardNumber::new(4)),
            false
        );
    }

    #[test]
    fn cmp_higher_card() {
        assert_eq!(Card::Ace == Card::Ace, true);
        assert_eq!(Card::Ace == Card::Joker, false);
        assert_eq!(Card::Queen < Card::King, true);
        assert_eq!(Card::Ace > Card::King, true);
        assert_eq!(Card::Number(CardNumber::new(8)) < Card::King, true);
    }

    #[test]
    fn from_char() {
        assert_eq!(Card::from_char('Q'), Card::Queen);
        assert_eq!(Card::from_char('T'), Card::Number(CardNumber::new(10)));
        assert_eq!(Card::from_char('A'), Card::Ace);
        assert_eq!(Card::from_char('5'), Card::Number(CardNumber::new(5)));
    }

    #[test]
    fn cmp_from_char_5_7() {
        assert!(Card::from_char('5') < Card::from_char('7'));
    }

    #[test]
    #[should_panic]
    fn from_char_panic() {
        Card::from_char('W');
    }
}
