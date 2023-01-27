// Max Fierro, maxfierro@berkeley.edu
// Friday January 20th, 2023


use super::{Game, Outcome};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use bimap::BiMap;
use uuid::Uuid;


pub const GAME_NAME: &str = "Zero-by-1-or-2";
pub const GAME_DESCRIPTION: &str =
"Zero-by-one-or-two is a game where there are N coins, and two players must
take turns removing one or two coins at a time until there are exactly zero
coins left. The player who removes the last coins remaining wins, meaning
that the player whose turn it is when there are zero coins remaining loses.";


#[derive(Eq, Hash, PartialEq, Clone, Copy, EnumIter)]
pub enum Move {
    One,
    Two
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
    fn solve(&self) -> Outcome {
        match self.coins_left() % 3 {
            1 => Outcome::Loss,
            _ => Outcome::Win
        }
    }

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

    fn undo(&mut self) {
        match self.stack.pop().expect("Expected move, found nothing.") {
            Move::One => self.coins += 1,
            Move::Two => self.coins += 2
        }
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

    fn state(&self) -> i32 {
        self.coins
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.coins <= 0 {
            Some(Outcome::Loss)
        } else {
            None
        }
    }
}