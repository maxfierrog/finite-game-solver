// Max Fierro, maxfierro@berkeley.edu
// Friday January 21st, 2023


use super::{Game, Outcome};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use bimap::BiMap;
use uuid::Uuid;


pub const GAME_NAME: &str = "Zero-by-1-3-or-4";
pub const GAME_DESCRIPTION: &str =
"Zero-by-one-or-three-or-four is a game where there are N coins, and two
players take turns removing one or three or four coins at a time until there
are zero coins left. The player who removes the last coins remaining wins,
and the player whose turn it is when there are no coins remaining loses.";


#[derive(Eq, Hash, PartialEq, Clone, Copy, EnumIter)]
pub enum Move {
    One,
    Three,
    Four
}


pub struct Session {
    coins: i32,
    moves: BiMap<Uuid, Move>,
    stack: Vec<Move>
}

impl Session {
    // FIXME: Return Result<Self, Error> instead, and have the error case
    // be when it is handed a non-positive amount of coins.
    pub fn new(coins: i32) -> Self {
        if coins < 0 { 
            panic!("Non-positive number of coins.");
        }
        let mut moves: BiMap<Uuid, Move> = BiMap::new();
        for mv in Move::iter() {
            moves.insert(Uuid::new_v4(), mv);
        }
        Session {
            coins,
            moves,
            stack: Vec::new()
        }
    }

    pub fn coins_left(&self) -> i32 {
        self.coins
    }

    pub fn move_uuid(&self, mv: Move) -> Uuid {
        *self.moves.get_by_right(&mv).expect("Invalid move.")
    }
}

impl Game for Session {
    fn play(&mut self, mv: Uuid) {
        let mv = *self.moves.get_by_left(&mv).expect("Error finding move.");
        match mv {
            Move::One => {
                self.coins -= 1;
            },
            Move::Three => {
                if self.coins >= 3 {
                    self.coins -= 3;
                } else {
                    panic!("Illegal move!");
                }
            },
            Move::Four => {
                if self.coins >= 4 {
                    self.coins -= 4;
                } else {
                    panic!("Illegal move!");
                }
            }
        }
        self.stack.push(mv.clone());
    }

    fn undo(&mut self) {
        match self.stack.pop().expect("Expected move, found nothing.") {
            Move::One => self.coins += 1,
            Move::Three => self.coins += 3,
            Move::Four => self.coins += 4
        }
    }

    fn possible_moves(&self) -> Vec<Uuid> {
        let coins_left = self.coins_left();
        if coins_left >= 4 {
            vec![self.move_uuid(Move::One), 
                 self.move_uuid(Move::Three),
                 self.move_uuid(Move::Four)]
        } else if coins_left >= 3 {
            vec![self.move_uuid(Move::One), 
                 self.move_uuid(Move::Three)]
        } else if coins_left >= 1 {
            vec![self.move_uuid(Move::One)]
        } else {
            vec![]
        }
    }

    fn state(&self) -> i32 {
        self.coins
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.coins <= 0 {
            Some(Outcome::Loss(0))
        } else {
            None
        }
    }
}