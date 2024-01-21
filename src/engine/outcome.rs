use std::cmp::Ordering;

use super::cards::Cards;
use super::constants::*;

#[derive(Debug, PartialEq, Eq)]
pub enum OutcomeError {
    CardCountTooLow(Cards),
    HighestCardNotFound(Cards),
    KindNotFound(Cards),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    StraightFlush(Cards),
    FourOfAKind(Cards),
    FullHouse(Cards),
    Flush(Cards),
    Straight(Cards),
    ThreeOfAKind(Cards),
    TwoPair(Cards),
    Pair(Cards),
    HighCard(Cards),
}

impl Outcome {
    pub fn from_cards(cards: Cards) -> Result<Self, OutcomeError> {
        if cards.value().count_ones() < 5 {
            return Err(OutcomeError::CardCountTooLow(cards));
        }

        let mut cards = cards;
        let flush = cards.get_flush();
        let straight = cards.get_straight();

        if let (Some(flush), Some(straight)) = (flush, straight) {
            if flush == straight {
                return Ok(Outcome::StraightFlush(flush));
            }
        }

        let kinds = cards.get_kinds();

        if let Some(quads) = kinds.iter().find(|cards| cards.card_count() == 4) {
            cards.remove_cards(quads.value());
            let tail = cards.get_highest(1).unwrap();

            return Ok(Outcome::FourOfAKind(Cards::from(
                quads.value() | tail.value(),
            )));
        }

        if let Some(trips) = kinds.iter().find(|cards| cards.card_count() == 3) {
            if let Some(pair) = kinds
                .iter()
                .find(|cards| cards != &trips && cards.card_count() >= 2)
            {
                return Ok(Outcome::FullHouse(Cards::from(
                    trips.value() | pair.value(),
                )));
            }
        }

        if let Some(flush) = flush {
            return Ok(Outcome::Flush(flush));
        }

        if let Some(straight) = straight {
            return Ok(Outcome::Straight(straight));
        }

        if let Some(trips) = kinds.iter().find(|cards| cards.card_count() == 3) {
            cards.remove_cards(trips.value());
            let tail = cards.get_highest(2).unwrap();

            return Ok(Outcome::ThreeOfAKind(Cards::from(
                trips.value() | tail.value(),
            )));
        }

        if kinds.len() >= 2 {
            let pair1 = kinds.get(0).ok_or(OutcomeError::KindNotFound(cards))?;
            let pair2 = kinds.get(1).ok_or(OutcomeError::KindNotFound(cards))?;

            cards.remove_cards(pair1.value());
            cards.remove_cards(pair2.value());
            let tail = cards.get_highest(1).unwrap();

            return Ok(Outcome::TwoPair(Cards::from(
                pair1.value() | pair2.value() | tail.value(),
            )));
        }

        if let Some(pair) = kinds.first() {
            cards.remove_cards(pair.value());
            let tail = cards.get_highest(3).unwrap();

            return Ok(Outcome::Pair(Cards::from(pair.value() | tail.value())));
        }

        let high_card = cards
            .get_highest(5)
            .ok_or(OutcomeError::HighestCardNotFound(cards))?;

        Ok(Outcome::HighCard(high_card))
    }
}

impl PartialOrd for Outcome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Outcome::StraightFlush(self_cards), Outcome::StraightFlush(other_cards)) => {
                let mut self_cards = self_cards.clone();
                if self_cards.has_any(ACE) && self_cards.has_any(FIVE) .clone(){
                    self_cards.remove_cards(self_cards.value() & ACE);
                }

                let mut other_cards = *other_cards;
                if other_cards.has_any(ACE) && other_cards.has_any(FIVE) {
                    other_cards.remove_cards(other_cards.value() & ACE);
                }

                Some(self_cards.compare_rank(&other_cards))
            }
            (Outcome::StraightFlush(_), _) => Some(Ordering::Greater),
            (_, Outcome::StraightFlush(_)) => Some(Ordering::Less),
            (Outcome::FourOfAKind(self_cards), Outcome::FourOfAKind(other_cards)) => {
                if self_cards == other_cards {
                    return Some(Ordering::Equal);
                }

                let self_kinds = self_cards.get_kinds();
                let other_kinds = other_cards.get_kinds();

                let self_quads = self_kinds.iter().find(|cards| cards.card_count() == 4)?;
                let other_quads = other_kinds.iter().find(|cards| cards.card_count() == 4)?;

                let compare_quads = self_quads.compare_rank(other_quads);
                let Ordering::Equal = compare_quads else {
                    return Some(compare_quads);
                };

                let mut self_cards = self_cards.clone().clone();
                let mut other_cards = other_cards.clone().clone();

                self_cards.remove_cards(self_quads.value());
                other_cards.remove_cards(other_quads.value());

                Some(self_cards.compare_rank(&other_cards))
            }
            (Outcome::FourOfAKind(_), _) => Some(Ordering::Greater),
            (_, Outcome::FourOfAKind(_)) => Some(Ordering::Less),
            (Outcome::FullHouse(self_cards), Outcome::FullHouse(other_cards)) => {
                if self_cards == other_cards {
                    return Some(Ordering::Equal);
                }

                let self_kinds = self_cards.get_kinds();
                let other_kinds = other_cards.get_kinds();

                let self_trips = self_kinds.iter().find(|cards| cards.card_count() == 3)?;
                let other_trips = other_kinds.iter().find(|cards| cards.card_count() == 3)?;

                let trips_compare = self_trips.compare_rank(other_trips);
                let Ordering::Equal = trips_compare else {
                    return Some(trips_compare);
                };

                let self_pair = self_kinds.iter().find(|cards| cards.card_count() == 2)?;
                let other_pair = other_kinds.iter().find(|cards| cards.card_count() == 2)?;

                Some(self_pair.compare_rank(other_pair))
            }
            (Outcome::FullHouse(_), _) => Some(Ordering::Greater),
            (_, Outcome::FullHouse(_)) => Some(Ordering::Less),
            (Outcome::Flush(self_cards), Outcome::Flush(other_cards)) => {
                Some(self_cards.compare_rank(other_cards))
            }
            (Outcome::Flush(_), _) => Some(Ordering::Greater),
            (_, Outcome::Flush(_)) => Some(Ordering::Less),
            (Outcome::Straight(self_cards), Outcome::Straight(other_cards)) => {
                let mut self_cards = self_cards.clone();
                if self_cards.has_any(ACE) && self_cards.has_any(FIVE) .clone(){
                    self_cards.remove_cards(self_cards.value() & ACE);
                }

                let mut other_cards = *other_cards;
                if other_cards.has_any(ACE) && other_cards.has_any(FIVE) {
                    other_cards.remove_cards(other_cards.value() & ACE);
                }

                Some(self_cards.compare_rank(&other_cards))
            }
            (Outcome::Straight(_), _) => Some(Ordering::Greater),
            (_, Outcome::Straight(_)) => Some(Ordering::Less),
            (Outcome::ThreeOfAKind(self_cards), Outcome::ThreeOfAKind(other_cards)) => {
                if self_cards == other_cards {
                    return Some(Ordering::Equal);
                }

                let self_kinds = self_cards.get_kinds();
                let other_kinds = other_cards.get_kinds();

                let self_three = self_kinds.iter().find(|cards| cards.card_count() == 3)?;
                let other_three = other_kinds.iter().find(|cards| cards.card_count() == 3)?;

                let compare_three = self_three.compare_rank(other_three);
                let Ordering::Equal = compare_three else {
                    return Some(compare_three);
                };

                let mut self_cards = self_cards.clone();
                let mut other_cards = other_cards.clone();

                self_cards.remove_cards(self_three.value());
                other_cards.remove_cards(other_three.value());

                Some(self_cards.compare_rank(&other_cards))
            }
            (Outcome::ThreeOfAKind(_), _) => Some(Ordering::Greater),
            (_, Outcome::ThreeOfAKind(_)) => Some(Ordering::Less),
            (Outcome::TwoPair(self_cards), Outcome::TwoPair(other_cards)) => {
                if self_cards == other_cards {
                    return Some(Ordering::Equal);
                }

                let self_kinds = self_cards.get_kinds();
                let other_kinds = other_cards.get_kinds();

                let self_pair1 = self_kinds.first()?;
                let other_pair1 = other_kinds.first()?;

                let compare_pair1 = self_pair1.compare_rank(other_pair1);
                let Ordering::Equal = compare_pair1 else {
                    return Some(compare_pair1);
                };

                let self_pair2 = self_kinds.get(1)?;
                let other_pair2 = other_kinds.get(1)?;

                let compare_pair2 = self_pair2.compare_rank(other_pair2);
                let Ordering::Equal = compare_pair2 else {
                    return Some(compare_pair2);
                };

                let mut self_cards = self_cards.clone();
                let mut other_cards = other_cards.clone();

                self_cards.remove_cards(self_pair1.value());
                self_cards.remove_cards(self_pair2.value());

                other_cards.remove_cards(other_pair1.value());
                other_cards.remove_cards(other_pair2.value());

                Some(self_cards.compare_rank(&other_cards))
            }
            (Outcome::TwoPair(_), _) => Some(Ordering::Greater),
            (_, Outcome::TwoPair(_)) => Some(Ordering::Less),
            (Outcome::Pair(self_cards), Outcome::Pair(other_cards)) => {
                if self_cards == other_cards {
                    return Some(Ordering::Equal);
                }

                let self_kinds = self_cards.get_kinds();
                let other_kinds = other_cards.get_kinds();

                let self_pair = self_kinds.first()?;
                let other_pair = other_kinds.first()?;

                let compare_pair = self_pair.compare_rank(other_pair);
                let Ordering::Equal = compare_pair else {
                    return Some(compare_pair);
                };

                let mut self_cards = self_cards.clone();
                let mut other_cards = other_cards.clone();

                self_cards.remove_cards(self_pair.value());
                other_cards.remove_cards(other_pair.value());

                Some(self_cards.compare_rank(&other_cards))
            }
            (Outcome::Pair(_), _) => Some(Ordering::Greater),
            (_, Outcome::Pair(_)) => Some(Ordering::Less),
            (Outcome::HighCard(self_cards), Outcome::HighCard(other_cards)) => {
                Some(self_cards.compare_rank(other_cards))
            }
        }
    }
}

impl Ord for Outcome {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::engine::{cards::Cards, constants::*};

    use super::Outcome;

    #[test]
    fn test_outcome_ordering() {
        let mut outcomes: Vec<Outcome> = vec![
            Outcome::TwoPair(Cards::default()),
            Outcome::HighCard(Cards::default()),
            Outcome::ThreeOfAKind(Cards::default()),
            Outcome::Straight(Cards::default()),
            Outcome::FourOfAKind(Cards::default()),
            Outcome::FullHouse(Cards::default()),
            Outcome::Flush(Cards::default()),
            Outcome::Pair(Cards::default()),
            Outcome::StraightFlush(Cards::default()),
        ];

        outcomes.sort();
        outcomes.reverse();

        let outcomes_test: Vec<Outcome> = vec![
            Outcome::StraightFlush(Cards::default()),
            Outcome::FourOfAKind(Cards::default()),
            Outcome::FullHouse(Cards::default()),
            Outcome::Flush(Cards::default()),
            Outcome::Straight(Cards::default()),
            Outcome::ThreeOfAKind(Cards::default()),
            Outcome::TwoPair(Cards::default()),
            Outcome::Pair(Cards::default()),
            Outcome::HighCard(Cards::default()),
        ];

        assert_eq!(outcomes_test, outcomes);
    }

    #[test]
    fn test_straight_flush_ordering() {
        let straight_flush1: Outcome = Outcome::from_cards(Cards::from(
            ACE & SPADE | KING & SPADE | QUEEN & SPADE | JACK & SPADE | TEN & SPADE,
        ))
        .unwrap();

        let straight_flush2: Outcome = Outcome::from_cards(Cards::from(
            ACE & HEART | KING & HEART | QUEEN & HEART | JACK & HEART | TEN & HEART,
        ))
        .unwrap();

        let straight_flush3: Outcome = Outcome::from_cards(Cards::from(
            KING & SPADE | QUEEN & SPADE | JACK & SPADE | TEN & SPADE | NINE & SPADE,
        ))
        .unwrap();

        let straight_flush4: Outcome = Outcome::from_cards(Cards::from(
            QUEEN & SPADE | JACK & SPADE | TEN & SPADE | NINE & SPADE | EIGHT & SPADE,
        ))
        .unwrap();

        let straight_flush5: Outcome = Outcome::from_cards(Cards::from(
            SIX & SPADE | FIVE & SPADE | FOUR & SPADE | THREE & SPADE | TWO & SPADE,
        ))
        .unwrap();

        assert_eq!(straight_flush1.cmp(&straight_flush2), Ordering::Equal);
        assert_eq!(straight_flush1.cmp(&straight_flush3), Ordering::Greater);
        assert_eq!(straight_flush1.cmp(&straight_flush4), Ordering::Greater);
        assert_eq!(straight_flush1.cmp(&straight_flush5), Ordering::Greater);
        assert_eq!(straight_flush2.cmp(&straight_flush1), Ordering::Equal);
        assert_eq!(straight_flush2.cmp(&straight_flush3), Ordering::Greater);
        assert_eq!(straight_flush2.cmp(&straight_flush4), Ordering::Greater);
        assert_eq!(straight_flush2.cmp(&straight_flush5), Ordering::Greater);
        assert_eq!(straight_flush3.cmp(&straight_flush1), Ordering::Less);
        assert_eq!(straight_flush3.cmp(&straight_flush2), Ordering::Less);
        assert_eq!(straight_flush3.cmp(&straight_flush4), Ordering::Greater);
        assert_eq!(straight_flush3.cmp(&straight_flush5), Ordering::Greater);
        assert_eq!(straight_flush4.cmp(&straight_flush1), Ordering::Less);
        assert_eq!(straight_flush4.cmp(&straight_flush2), Ordering::Less);
        assert_eq!(straight_flush4.cmp(&straight_flush3), Ordering::Less);
        assert_eq!(straight_flush4.cmp(&straight_flush5), Ordering::Greater);
        assert_eq!(straight_flush5.cmp(&straight_flush1), Ordering::Less);
        assert_eq!(straight_flush5.cmp(&straight_flush2), Ordering::Less);
        assert_eq!(straight_flush5.cmp(&straight_flush3), Ordering::Less);
        assert_eq!(straight_flush5.cmp(&straight_flush4), Ordering::Less);
    }

    #[test]
    fn test_four_of_a_kind_ordering() {
        let quad1: Outcome = Outcome::from_cards(Cards::from(
            ACE & HEART | QUEEN & SPADE | QUEEN & HEART | QUEEN & DIAMOND | QUEEN & CLUB,
        ))
        .unwrap();

        let quad2: Outcome = Outcome::from_cards(Cards::from(
            ACE & HEART | FOUR & SPADE | FOUR & HEART | FOUR & DIAMOND | FOUR & CLUB,
        ))
        .unwrap();

        let quad3: Outcome = Outcome::from_cards(Cards::from(
            EIGHT & HEART | FOUR & SPADE | FOUR & HEART | FOUR & DIAMOND | FOUR & CLUB,
        ))
        .unwrap();

        let quad4: Outcome = Outcome::from_cards(Cards::from(
            EIGHT & DIAMOND | FOUR & SPADE | FOUR & HEART | FOUR & DIAMOND | FOUR & CLUB,
        ))
        .unwrap();

        assert_eq!(quad1.cmp(&quad2), Ordering::Greater);
        assert_eq!(quad1.cmp(&quad3), Ordering::Greater);
        assert_eq!(quad1.cmp(&quad4), Ordering::Greater);
        assert_eq!(quad2.cmp(&quad1), Ordering::Less);
        assert_eq!(quad2.cmp(&quad3), Ordering::Greater);
        assert_eq!(quad2.cmp(&quad4), Ordering::Greater);
        assert_eq!(quad3.cmp(&quad1), Ordering::Less);
        assert_eq!(quad3.cmp(&quad2), Ordering::Less);
        assert_eq!(quad3.cmp(&quad4), Ordering::Equal);
        assert_eq!(quad4.cmp(&quad1), Ordering::Less);
        assert_eq!(quad4.cmp(&quad2), Ordering::Less);
        assert_eq!(quad4.cmp(&quad3), Ordering::Equal);
    }
}
