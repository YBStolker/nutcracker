use rand;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;

use crate::util::bit_iterator::IntoFromLeftBitIterator;
use crate::util::bit_iterator::IntoFromRightBitIterator;

use super::constants::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardsError {
    InvalidCards(Cards),
    NoCards,
}

impl Display for CardsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for CardsError {}

impl From<u64> for Cards {
    fn from(value: u64) -> Self {
        Cards { _value: value }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Cards {
    _value: u64,
}

impl Cards {
    pub fn new(value: u64) -> Result<Cards, CardsError> {
        let cards = Cards { _value: value };
        if Cards::is_valid(value) {
            Ok(cards)
        } else {
            Err(CardsError::InvalidCards(cards))
        }
    }

    pub fn value(&self) -> u64 {
        self._value
    }

    pub fn set_value(&mut self, new_value: u64) {
        self._value = new_value;
    }

    pub fn add_cards(&mut self, cards: u64) {
        self.set_value(self._value | cards)
    }

    pub fn remove_cards(&mut self, cards: u64) {
        self.add_cards(cards);
        self.set_value(self._value ^ cards);
    }

    pub fn is_valid(cards: u64) -> bool {
        cards != 0 && (FULL_DECK & cards) == cards
    }

    pub fn card_count(&self) -> u32 {
        self.value().count_ones()
    }

    pub fn take_random(&mut self) -> Result<Cards, CardsError> {
        let card_count = self.card_count();
        if card_count == 0 {
            return Err(CardsError::NoCards);
        }

        let random_number: f32 = rand::random();
        let card_i = (random_number * card_count as f32) as usize;
        for (_, c) in self
            .value()
            .iter_from_right()
            .take(card_i + 1)
            .enumerate()
            .skip(card_i)
        {
            self._value ^= c;
            return Ok(Cards::from(c));
        }

        unreachable!()
    }

    pub fn has_any(&self, cards: u64) -> bool {
        self.value() & cards != 0
    }

    pub fn get_highest(&self, num: usize) -> Option<Self> {
        if num == 0 || self.card_count() < num as u32 {
            return None;
        }

        let mut highest = 0;
        let mut card_count = 0;
        for card in self.value().iter_from_left() {
            highest |= card;
            card_count += 1;
            if card_count >= num {
                // Is highest.count_ones() == num faster?
                return Some(Cards { _value: highest });
            }
        }

        None
    }

    pub fn compare_rank(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        for rank in RANKS {
            if (self.value() & rank) != 0 && (other.value() & rank) == 0 {
                return Ordering::Greater;
            } else if (self.value() & rank) == 0 && (other.value() & rank) != 0 {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }

    pub fn get_flush(&self) -> Option<Self> {
        if let Ok(spades) = Cards::new(SPADE & self.value()) {
            if let Some(spades) = spades.get_highest(5) {
                return Some(spades);
            }
        }

        if let Ok(hearts) = Cards::new(HEART & self.value()) {
            if let Some(hearts) = hearts.get_highest(5) {
                return Some(hearts);
            }
        }

        if let Ok(diamonds) = Cards::new(DIAMOND & self.value()) {
            if let Some(diamonds) = diamonds.get_highest(5) {
                return Some(diamonds);
            }
        }

        if let Ok(clubs) = Cards::new(CLUB & self.value()) {
            if let Some(clubs) = clubs.get_highest(5) {
                return Some(clubs);
            }
        }

        None
    }

    pub fn get_straight(&self) -> Option<Self> {
        let ranks_len = RANKS.len();
        for i in 0..(ranks_len - 3) {
            let Ok(mut one) = Cards::new(RANKS[i] & self.value()) else {
                continue;
            };

            let Ok(mut two) = Cards::new(RANKS[i + 1] & self.value()) else {
                continue;
            };

            let Ok(mut three) = Cards::new(RANKS[i + 2] & self.value()) else {
                continue;
            };

            let Ok(mut four) = Cards::new(RANKS[i + 3] & self.value()) else {
                continue;
            };

            let five = if i + 4 < ranks_len {
                Cards::new(RANKS[i + 4] & self.value())
            } else {
                Cards::new(ACE & self.value()) // 5, 4, 3, 2, A is also a straight.
            };

            let Ok(mut five) = five else {
                continue;
            };

            if one.card_count() > 1 {
                one = one.get_highest(1)?;
            }

            if two.card_count() > 1 {
                two = two.get_highest(1)?;
            }

            if three.card_count() > 1 {
                three = three.get_highest(1)?;
            }

            if four.card_count() > 1 {
                four = four.get_highest(1)?;
            }

            if five.card_count() > 1 {
                five = five.get_highest(1)?;
            }

            return Some(Cards::from(
                one.value() | two.value() | three.value() | four.value() | five.value(),
            ));
        }

        None
    }

    pub fn get_kinds(&self) -> Vec<Self> {
        RANKS
            .iter()
            .filter_map(|rank| {
                ((self.value() & rank).count_ones() > 1).then_some(self.value() & rank)
            })
            .map(Cards::from)
            .collect()
    }
}

impl Debug for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cards({})", self)
    }
}

impl Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card_string = self
            .value()
            .iter_from_left()
            .filter_map(|card| {
                let rank_i = RANKS
                    .iter()
                    .enumerate()
                    .find(|(_, rank)| *rank & card != 0)
                    .map(|(i, _)| i)?;

                let suit_i = SUITS
                    .iter()
                    .enumerate()
                    .find(|(_, suit)| *suit & card != 0)
                    .map(|(i, _)| i)?;

                Some(format!("{}{}", RANK_NAMES[rank_i], SUIT_NAMES[suit_i]))
            })
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "[{}]", card_string)
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::outcome::Outcome;

    use super::*;

    #[test]
    fn test_take_random() {
        let mut cards = Cards::from(ACE & SPADE | FOUR & CLUB | KING & HEART | JACK & DIAMOND);

        for _ in 0..cards.card_count() {
            let taken_card = cards.take_random().unwrap();
            println!("{}", taken_card);
            assert!(taken_card.card_count() == 1)
        }

        println!("{}", cards);
        assert!(cards.card_count() == 0)
    }

    #[test]
    fn test_into() {
        let cards = Cards::from(ACE & SPADE | FOUR & HEART | EIGHT & CLUB | JACK & DIAMOND);
        println!("{}", cards)
    }

    #[test]
    fn test_straight_flush() {
        let hand = Cards::from(
            JACK & SPADE
                | TEN & SPADE
                | NINE & SPADE
                | EIGHT & SPADE
                | SEVEN & SPADE
                | THREE & HEART
                | KING & CLUB,
        );

        let straight_flush = Cards {
            _value: JACK & SPADE | TEN & SPADE | NINE & SPADE | EIGHT & SPADE | SEVEN & SPADE,
        };

        assert_eq!(
            Ok(Outcome::StraightFlush(straight_flush)),
            Outcome::from_cards(hand)
        );
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = Cards::from(
            TEN & SPADE
                | TEN & HEART
                | TEN & DIAMOND
                | TEN & CLUB
                | ACE & SPADE
                | FOUR & HEART
                | JACK & HEART,
        );

        let four_of_a_kind =
            Cards::from(TEN & SPADE | TEN & HEART | TEN & DIAMOND | TEN & CLUB | ACE & SPADE);

        assert_eq!(
            Ok(Outcome::FourOfAKind(four_of_a_kind)),
            Outcome::from_cards(hand)
        )
    }

    #[test]
    fn test_full_house() {
        let hand = Cards::from(
            FOUR & SPADE
                | FOUR & HEART
                | FOUR & DIAMOND
                | JACK & SPADE
                | JACK & CLUB
                | FIVE & HEART
                | FIVE & DIAMOND,
        );

        let full_house =
            Cards::from(FOUR & SPADE | FOUR & HEART | FOUR & DIAMOND | JACK & SPADE | JACK & CLUB);

        assert_eq!(
            Ok(Outcome::FullHouse(full_house)),
            Outcome::from_cards(hand)
        )
    }

    #[test]
    fn test_flush() {
        let hand = Cards::from(
            ACE & SPADE
                | TEN & SPADE
                | SEVEN & SPADE
                | FOUR & SPADE
                | THREE & SPADE
                | EIGHT & HEART
                | KING & CLUB,
        );

        let flush =
            Cards::from(ACE & SPADE | TEN & SPADE | SEVEN & SPADE | FOUR & SPADE | THREE & SPADE);

        assert_eq!(Ok(Outcome::Flush(flush)), Outcome::from_cards(hand));
    }

    #[test]
    fn test_straight() {
        let hand = Cards::from(
            TEN & CLUB
                | NINE & CLUB
                | EIGHT & HEART
                | SEVEN & DIAMOND
                | SIX & HEART
                | FIVE & CLUB
                | ACE & SPADE,
        );

        let straight =
            Cards::from(TEN & CLUB | NINE & CLUB | EIGHT & HEART | SEVEN & DIAMOND | SIX & HEART);

        assert_eq!(Ok(Outcome::Straight(straight)), Outcome::from_cards(hand));
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = Cards::from(
            FOUR & SPADE
                | FOUR & HEART
                | FOUR & DIAMOND
                | ACE & SPADE
                | KING & DIAMOND
                | JACK & CLUB
                | FIVE & HEART,
        );

        let three_of_a_kind = Cards::from(
            FOUR & SPADE | FOUR & HEART | FOUR & DIAMOND | ACE & SPADE | KING & DIAMOND,
        );

        assert_eq!(
            Ok(Outcome::ThreeOfAKind(three_of_a_kind)),
            Outcome::from_cards(hand)
        );
    }

    #[test]
    fn test_two_pair() {
        let hand = Cards::from(
            FOUR & SPADE
                | FOUR & HEART
                | JACK & DIAMOND
                | JACK & CLUB
                | ACE & SPADE
                | THREE & DIAMOND
                | THREE & HEART,
        );

        let two_pair =
            Cards::from(FOUR & SPADE | FOUR & HEART | JACK & DIAMOND | JACK & CLUB | ACE & SPADE);

        assert_eq!(Ok(Outcome::TwoPair(two_pair)), Outcome::from_cards(hand))
    }

    #[test]
    fn test_pair() {
        let hand = Cards::from(
            EIGHT & SPADE
                | EIGHT & HEART
                | ACE & DIAMOND
                | QUEEN & SPADE
                | FOUR & CLUB
                | TWO & DIAMOND
                | THREE & HEART,
        );

        let pair = Cards::from(
            EIGHT & SPADE | EIGHT & HEART | ACE & DIAMOND | QUEEN & SPADE | FOUR & CLUB,
        );

        assert_eq!(Ok(Outcome::Pair(pair)), Outcome::from_cards(hand))
    }

    #[test]
    fn test_high_card() {
        let hand = Cards::from(
            ACE & SPADE
                | KING & DIAMOND
                | QUEEN & SPADE
                | TEN & HEART
                | FOUR & CLUB
                | TWO & DIAMOND
                | THREE & HEART,
        );

        let high_card =
            Cards::from(ACE & SPADE | KING & DIAMOND | QUEEN & SPADE | TEN & HEART | FOUR & CLUB);

        assert_eq!(Ok(Outcome::HighCard(high_card)), Outcome::from_cards(hand))
    }
}
