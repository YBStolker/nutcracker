use crate::engine::{cards::Cards, constants::FULL_DECK};

use super::player::Player;

#[derive(Debug, Clone)]
pub enum GameEvent {
    Bet(u32),
    Flop(Cards),
    Turn(Cards),
    River(Cards),
}

#[derive(Debug, Default)]
pub struct Game {
    players: Vec<Player>,
    dealer: usize,
    deck: Cards,
    game_history: Vec<GameEvent>,
}

impl Game {
    pub fn new_round(&mut self) {
        self.deck = Cards::from(FULL_DECK);
        for p in self.players.iter() {
        }
    }

    pub fn add_player(&mut self, player: Player) -> () {
        self.players.push(player);
    }

    pub fn add_bet(&mut self, bet: u32) -> () {
        self.game_history.push(GameEvent::Bet(bet));

    }
}

