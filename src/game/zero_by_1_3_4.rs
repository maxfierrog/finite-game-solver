// Max Fierro, maxfierro@berkeley.edu
// Friday January 21st, 2023


use super::{Game, Outcome};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use bimap::BiMap;
use uuid::Uuid;


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
    fn solve(&self) -> Outcome {
        let mut curr = Outcome::Win;
        let mut n_1 = Outcome::Win;
        let mut n_2 = Outcome::Loss;
        let mut n_3 = Outcome::Win;
        let mut n_4 = Outcome::Loss;
        match self.coins_left() {
            0 => n_4,
            1 => n_3,
            2 => n_2,
            3 => n_1,
            4 => curr,
            _ => {
                let mut count = 4;
                while self.coins_left() > count {
                    count += 1;
                    n_4 = n_3;
                    n_3 = n_2;
                    n_2 = n_1;
                    n_1 = curr;
                    curr = if [n_1, n_3, n_4].contains(&Outcome::Loss) {
                        Outcome::Win
                    } else {
                        Outcome::Loss
                    };
                }
                curr
            }
        }
    }

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
            Some(Outcome::Loss)
        } else {
            None
        }
    }
}