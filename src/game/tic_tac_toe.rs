// Max Fierro, maxfierro@berkeley.edu
// Monday, January 23rd, 2023


pub mod board;


use super::{Game, Outcome};
use board::Board;
use bimap::BiMap;
use uuid::Uuid;


pub const GAME_NAME: &str = "Tic-Tac-Toe";
pub const GAME_DESCRIPTION: &str = 
"C'mon, you know tic-tac-toe, right?";


#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Move {
    X(Place),
    O(Place)
}


#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Place {
    i: i32,
    j: i32
}

pub struct Session {
    board: Board,
    moves: BiMap<Uuid, Move>,
    stack: Vec<Move>
}

impl Session {
    pub fn new(height: i32, width: i32, win: i32) -> Self {
        Session {
            board: Board::new(height, width, win),
            moves: Self::map_possible_moves(height, width),
            stack: Vec::new()
        }
    }

    fn map_possible_moves(height: i32, width: i32) -> BiMap<Uuid, Move> {
        let mut moves: BiMap<Uuid, Move> = BiMap::new();
        for i in 0..height {
            for j in 0..width {
                let place = Place {i, j};
                let circle = Move::O(place);
                let cross = Move::X(place);
                moves.insert(Uuid::new_v4(), circle);
                moves.insert(Uuid::new_v4(), cross);
            }
        }
        moves
    }

    fn move_from_uuid(&self, id: Uuid) -> Move {
        *self.moves.get_by_left(&id).unwrap()
    }

    fn retain_move_candidates(&self, v: &mut Vec<Uuid>) {
        if self.stack.len() % 2 == 0 {
            // O's turn, eliminate Xs from v
            v.retain(|&o| match self.move_from_uuid(o) {
                Move::O(_) => true,
                Move::X(_) => false
            });
        } else {
            // X's turn, eliminate Os from v
            v.retain(|&x| match self.move_from_uuid(x) {
                Move::X(_) => true,
                Move::O(_) => false
            });
        }
    }
}

impl Game for Session {
    fn play(&mut self, mv: Uuid) {
        let mv = self.moves.get_by_left(&mv).expect("Could not find move.");
        match mv {
            Move::O(place) => {
                if let None = self.board.symbol_at(place.i, place.j) {
                    self.board.place(Some(false), place.i, place.j);
                } else {
                    panic!("Attempted illegal move.");
                }
            },
            Move::X(place) => {
                if let None = self.board.symbol_at(place.i, place.j) {
                    self.board.place(Some(true), place.i, place.j);
                } else {
                    panic!("Attempted illegal move.");
                }
            }
        }
        self.stack.push(*mv);
    }

    fn undo(&mut self) {
        let mv = self.stack.pop().expect("No move to pop!");
        let place = match mv { 
            Move::X(place) => place,
            Move::O(place) => place
        };
        self.board.place(None, place.i, place.j);
    }

    fn possible_moves(&self) -> Vec<Uuid> {
        let mut result: Vec<Uuid> = Vec::new();
        for (id, mv) in self.moves.iter() {
            match mv {
                Move::O(place) => {
                    if let None = self.board.symbol_at(place.i, place.j) {
                        result.push(*id);
                    }
                },
                Move::X(place) => {
                    if let None = self.board.symbol_at(place.i, place.j) {
                        result.push(*id);
                    }
                }
            }
        }
        self.retain_move_candidates(&mut result);
        result
    }

    fn state(&self) -> i32 {
        self.board.hash()
    }

    fn outcome(&self) -> Option<Outcome> {
        self.board.outcome()
    }
}