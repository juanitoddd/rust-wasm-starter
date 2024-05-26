
use crate::game::utils::coord::Coord;
use crate::game::utils::{PlayerNumber, player_number_match};
use super::logic::player::Player;
use super::logic::board::Board;

use js_sys::Math;

#[derive(Debug)]
pub struct Manager {
    board: Board,
    player1: Player,
    player2: Player,
}

impl Manager {
    pub fn new(name1: String, name2: String) -> Self {
        let board = Board::new();
        let player1 = Player::new(name1, PlayerNumber::One);
        let player2 = Player::new(name2, PlayerNumber::Two); 
        Manager{ player1, player2, board }
    }
}

