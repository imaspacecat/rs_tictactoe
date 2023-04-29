use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Copy, Clone)]
pub enum Player {
    X,
    O
}
impl Debug for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "Cross"),
            Player::O => write!(f, "Nought")
        }
    }
}

impl Player {
    pub fn opposite(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X
        }
    }
}