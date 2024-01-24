use gameplay::game::Game;
use gameplay::player::Player;

mod engine;
mod gameplay;
mod util;

fn main() {
    let mut game = Game::default();
    let players = vec![
        Player::new("Yannick", 200000),
        Player::new("Yan", 300000),
        Player::new("Nick", 400000),
    ];

    for p in players {
        game.add_player(p);
    }

    println!("Game new_round");
    game.new_round();
}
