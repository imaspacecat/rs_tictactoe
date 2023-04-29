use std::fmt::{Display, Formatter};
use crate::board::Board;
use crate::game::GameState::Playing;
use crate::player::Player;

#[derive(PartialEq)]
pub enum GameState {
    Victory(Player),
    Tie,
    Playing
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::Victory(Player::X) => write!(f, "Cross's Victory"),
            GameState::Victory(Player::O) => write!(f, "Nought's Victory"),
            GameState::Tie => write!(f, "Tie"),
            Playing => write!(f, "Playing")
        }
    }
}

pub struct Game {
    pub board: Board,
    pub current_player: Player
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}current player: {}\ngame status: {}", self.board, self.current_player,
               self.get_game_status())
    }
}

impl Game {
    pub fn do_turn(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        if !self.board.make_move(self.current_player, x, y) {
            return Err("invalid move");
        }
        self.current_player = self.current_player.opposite();
        Ok(())
    }

    pub fn get_game_status(&self) -> GameState {
        if self.board.is_victory() {
            return GameState::Victory(self.current_player.opposite());
        } else if self.board.is_tie() {
            return GameState::Tie;
        }
        Playing
    }

    pub fn new_with_player(starting_player: Player) -> Game {
        Game {
            board: Board::new(),
            current_player: starting_player
        }
    }

    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: Player::X
        }
    }
}