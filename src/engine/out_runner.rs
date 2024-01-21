use crate::util::all_bit_combo_iterator::IntoAllBitIterator;

use super::cards::Cards;
use super::outcome::Outcome;
use super::outcome::OutcomeError;

#[derive(Debug)]
pub enum RunoutError {
    InvalidHand(Cards),
    InvalidTable(Cards),
    CardCountTooLow(Cards, Cards),
    InvalidOutcome(Cards),
}

impl From<OutcomeError> for RunoutError {
    fn from(value: OutcomeError) -> Self {
        match value {
            OutcomeError::CardCountTooLow(cards) => RunoutError::InvalidOutcome(cards),
            OutcomeError::HighestCardNotFound(cards) => RunoutError::InvalidOutcome(cards),
            OutcomeError::KindNotFound(cards) => RunoutError::InvalidOutcome(cards),
        }
    }
}

#[derive(Debug)]
pub struct Chance {
    win: f32,
    tie: f32,
    loss: f32,
}

impl Default for Chance {
    fn default() -> Self {
        Self {
            win: 0.0,
            tie: 0.0,
            loss: 0.0,
        }
    }
}

impl Chance {
    pub fn normalize(self) -> Chance {
        let total = self.win + self.tie + self.loss;
        if total != 0.0 {
            Chance {
                win: self.win / total,
                tie: self.tie / total,
                loss: self.loss / total,
            }
        } else {
            Chance::default()
        }
    }

    pub fn add(&mut self, other: Self) {
        self.win += other.win;
        self.tie += other.tie;
        self.loss += other.loss;
    }
}

pub fn runout(player: Cards, table: Cards, deck: Cards) -> Result<Chance, RunoutError> {
    if player.card_count() != 2 {
        Err(RunoutError::InvalidHand(player))?;
    }

    let mut chance = Chance::default();
    for new_table_cards in deck
        .value()
        .iter_all_combos(5 - table.card_count() as usize)
    {
        let new_deck = Cards::from(deck.value() | new_table_cards);
        let new_player = Cards::from(player.value() | new_table_cards);

        let player_outcome: Outcome = Outcome::from_cards(new_player).map_err(RunoutError::from)?;

        for opponent_cards in new_deck.value().iter_all_combos(2) {
            let opponent_cards = Cards::from(opponent_cards | new_table_cards);

            let opponent_outcome: Outcome =
                Outcome::from_cards(opponent_cards).map_err(RunoutError::from)?;

            match player_outcome.cmp(&opponent_outcome) {
                std::cmp::Ordering::Greater => chance.win += 1f32,
                std::cmp::Ordering::Equal => chance.tie += 1f32,
                std::cmp::Ordering::Less => chance.loss += 1f32,
            }
        }
    }

    Ok(chance.normalize())
}

#[cfg(test)]
mod tests {
    use crate::engine::cards::Cards;
    use crate::engine::constants::*;

    #[test]
    fn test_runout_table() {
        let hand = Cards::from(ACE & CLUB | ACE & DIAMOND);
        let table = Cards::from(ACE & SPADE | ACE & HEART | KING & DIAMOND);
        let deck = Cards::from(
            QUEEN & CLUB
                | QUEEN & DIAMOND
                | QUEEN & SPADE
                | QUEEN & HEART
                | JACK & CLUB
                | JACK & DIAMOND
                | JACK & SPADE
                | JACK & HEART
                | TEN & CLUB
                | TEN & DIAMOND
                | TEN & SPADE
                | TEN & HEART,
        );

        // let chance = runout_table(hand, table, deck).unwrap();

        // println!("chance {:?}", chance)
    }

    #[test]
    fn test_testing() {
        println!("I am testing!");
    }
}
