use crate::game::Game;

mod color;
mod bitboard;
mod square;
mod pieces;
mod game;
mod position;
mod constants;
mod fen;
mod error;

fn main() {
    // you just lost the game
    let game = Game::new();
    game.print();
}
