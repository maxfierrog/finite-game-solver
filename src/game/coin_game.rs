// Max Fierro, maxfierro@berkeley.edu
// Friday January 20th, 2022

use super::{Game, Outcome};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use bimap::BiMap;
use uuid::Uuid;


#[derive(Eq, Hash, PartialEq, Clone, Copy, EnumIter)]
pub enum Move {
    One,
    Two
}


pub struct CoinGame {
    coins: i32,
    moves: BiMap<Uuid, Move>,
    stack: Vec<Move>
}

impl CoinGame {
    pub fn new(coins: i32) -> Self {
        let mut moves: BiMap<Uuid, Move> = BiMap::new();
        for mv in Move::iter() {
            moves.insert(Uuid::new_v4(), mv);
        }
        CoinGame {
            coins,
            moves,
            stack: Vec::new()
        }
    }

    fn coins_left(&self) -> i32 {
        self.coins
    }

    pub fn move_uuid(&self, mv: Move) -> Uuid {
        *self.moves.get_by_right(&mv).expect("Invalid move.")
    }
}

impl Game for CoinGame {
    fn play(&mut self, mv: Uuid) {
        let mv = *self.moves.get_by_left(&mv).expect("Error finding move.");
        match mv {
            Move::One => {
                self.coins -= 1;
            },
            Move::Two => {
                if self.coins >= 2 {
                    self.coins -= 2;
                } else {
                    panic!("Illegal move!");
                }
            }
        }
        self.stack.push(mv.clone());
    }

    fn possible_moves(&self) -> Vec<Uuid> {
        let coins_left = self.coins_left();
        if coins_left > 1 {
            vec![self.move_uuid(Move::One), self.move_uuid(Move::Two)]
        } else if coins_left == 1 {
            vec![self.move_uuid(Move::One)]
        } else {
            vec![]
        }
    }

    fn undo(&mut self) {
        match self.stack.pop().expect("Expected move, found nothing.") {
            Move::One => self.coins += 1,
            Move::Two => self.coins += 2
        }
    }

    fn encode_state(&self) -> i32 {
        self.coins
    }

    fn evaluate_state(&self) -> Option<Outcome> {
        if self.coins <= 0 {
            Some(Outcome::Loss)
        } else {
            None
        }
    }
}