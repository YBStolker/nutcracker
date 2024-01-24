use crate::engine::{cards::Cards, constants::FULL_DECK};

use super::player::Player;

#[derive(Debug, Clone)]
pub enum GameEvent {
    StartStack(u32),
    Bet(u32),
    Flop(Cards),
    Turn(Cards),
    River(Cards),
}

#[derive(Debug, Default)]
pub struct Game {
    players: Vec<Player>,
    hands: Vec<Cards>,
    dealer: usize,
    deck: Cards,
    game_history: Vec<GameEvent>,
}

impl Game {
    pub fn new_round(&mut self) -> bool {
        if self.players.len() < 2 {
            return false;
        }

        let first_player = self.players.remove(0);
        self.players.push(first_player);

        for i in 0..self.players.len() {
            let player = self
                .players
                .get((i + self.dealer) % self.players.len())
                .unwrap();

            self.game_history
                .push(GameEvent::StartStack(player.stack()))
        }

        self.deck = Cards::from(FULL_DECK);

        for i in 0..self.players.len() * 2 {
            if let Some(hand) = self.hands.get_mut(i % self.players.len()) {
                hand.add_cards(self.deck.take_random().unwrap().value());
            } else {
                self.hands.push(self.deck.take_random().unwrap())
            }
        }

        true
    }

    pub fn add_player(&mut self, player: Player) -> bool {
        if player.stack() < 100 {
            return false;
        }

        self.players.push(player);
        true
    }

    pub fn add_bet(&mut self, bet: u32) {
        self.game_history.push(GameEvent::Bet(bet));
    }
}
