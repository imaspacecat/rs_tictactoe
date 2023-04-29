use std::fmt::{Display, Formatter};
use crate::player::Player;

pub struct Board {
    board: [[Option<Player>; 3]; 3],
}


impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for (i, row) in self.board.iter().enumerate() {
            for x in row {
                match x {
                    Some(Player::O) => output.push_str("O"),
                    Some(Player::X) => output.push_str("X"),
                    None => output.push_str("-") // &(i * 3 + j + 1).to_string()
                }
            }
            output.push_str(&*("\t".to_owned() + &(i * 3 + 3 + (i * 3 + 2) * 10 + (i * 3 + 1) * 100)
                .to_string() + "\n"));
        }
        write!(f, "{}", output)
    }
}

impl Board {
    pub fn make_move(&mut self, p: Player, x: usize, y: usize) -> bool {
        if self.is_legal_move(x, y) {
            self.board[x][y] = Some(p);
            return true;
        }
        false
    }

    fn is_legal_move(&self, x: usize, y: usize) -> bool {
        if self.board[x][y] == Some(Player::X) || self.board[x][y] == Some(Player::O) {
            false
        } else {
            true
        }
    }

    pub fn is_victory(&self) -> bool {
        // check rows
        for row in &self.board {
            if row[0] == row[1] && row[1] == row[2] && row[0] != None {
                return true;
            }
        }

        // check columns
        for i in 0..2 as usize {
            if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i]
                && self.board[0][i] != None  {
                return true;
            }
        }

        // check diagonal
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]
            && self.board[1][1] != None {
            return true;
        } else if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0]
            && self.board[1][1] != None {
            return true;
        }

        false
    }

    pub fn is_tie(&self) -> bool {
        for row in &self.board {
            for x in row {
                if *x == None {
                    return false;
                }
            }
        }
        true
    }

    pub fn new() -> Board {
        Board {
            board: [
                [None, None, None],
                [None, None, None],
                [None, None, None]
            ]
        }
    }
}
