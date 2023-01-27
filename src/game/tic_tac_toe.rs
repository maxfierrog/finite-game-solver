// Max Fierro, maxfierro@berkeley.edu
// Monday, January 23rd, 2023


use super::{Game, Outcome};
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
    x: i32,
    y: i32
}


pub struct Session {
    board: [[Option<bool>; 3]; 3],
    moves: BiMap<Uuid, Move>,
    stack: Vec<Move>
}

impl Session {
    pub fn new() -> Self {
        Session {
            board: [[None; 3]; 3],
            moves: Self::map_possible_moves(),
            stack: Vec::new()
        }
    }

    fn map_possible_moves() -> BiMap<Uuid, Move> {
        let mut moves: BiMap<Uuid, Move> = BiMap::new();
        for x in 0..3 {
            for y in 0..3 {
                let place = Place {x, y};
                let circle = Move::O(place);
                let cross = Move::X(place);
                moves.insert(Uuid::new_v4(), circle);
                moves.insert(Uuid::new_v4(), cross);
            }
        }
        moves
    }

    fn symbol_at(&self, x: i32, y: i32) -> Option<bool> {
        if x > 2 || x < 0 || y > 2 || y < 0 {
            panic!("Attempted to access tile outside board.");
        }
        self.board[x as usize][y as usize]
    }

    fn print_board(&self) {
        for i in self.board {
            for j in i {
                if let Some(c) = j {
                    if c {
                        print!("X ");
                    } else {
                        print!("O ")
                    }
                } else {
                    print!("  ");
                }
            }
            print!("\n");
        }
        println!("-----");
        for i in self.stack.iter() {
            let place = match i { 
                Move::O(place) => { print!("O: "); place },
                Move::X(place) => { print!("X: "); place }
            };
            print!("({}, {}) => ", place.x, place.y);
        }
        println!("\n");
    }

    /* UTILITIES */

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
    fn solve(&self) -> Outcome {
        Outcome::Tie
    }

    fn play(&mut self, mv: Uuid) {
        let mv = self.moves.get_by_left(&mv).expect("Could not find move.");
        match mv {
            Move::O(place) => {
                if let None = self.symbol_at(place.x, place.y) {
                    self.board[place.x as usize][place.y as usize] = Some(false);
                } else {
                    panic!("Attempted illegal move.");
                }
            },
            Move::X(place) => {
                if let None = self.symbol_at(place.x, place.y) {
                    self.board[place.x as usize][place.y as usize] = Some(true);
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
        self.board[place.x as usize][place.y as usize] = None;
    }

    fn possible_moves(&self) -> Vec<Uuid> {
        let mut result: Vec<Uuid> = Vec::new();
        for (id, mv) in self.moves.iter() {
            match mv {
                Move::O(place) => {
                    if let None = self.symbol_at(place.x, place.y) {
                        result.push(*id);
                    }
                },
                Move::X(place) => {
                    if let None = self.symbol_at(place.x, place.y) {
                        result.push(*id);
                    }
                }
            }
        }
        self.retain_move_candidates(&mut result);
        result
    }

    fn state(&self) -> i32 {
        let mut counter = 0;
        let mut result = 0;
        for x in 0..3 {
            for y in 0..3 {
                let curr: i32;
                match self.symbol_at(x, y) {
                    Some(true) => { curr = 3; },
                    Some(false) => { curr = 2; },
                    None => { curr = 1; }
                }
                result += curr * 10_i32.pow(counter);
                counter += 1;
            }
        }
        result
    }

    fn outcome(&self) -> Option<Outcome> {
        let mut h_win = false;
        let mut v_win = false;
        let mut d_win = false;

        // Horizontal wins
        for x in 0..3 {
            let mut win = false;
            if let Some(first) = self.symbol_at(x, 0) {
                win = self.board[x as usize].iter().all(|&s| s == Some(first));
            }
            if win {
                h_win = true;
                break;
            }
        }

        // Vertical wins
        for y in 0..3 {
            let mut win = true;
            let first = self.symbol_at(0, y);
            for x in 0..3 {
                if let None = first {
                    win = false;
                    break
                }
                if self.symbol_at(x, y) != first {
                    win = false;
                    break
                }
            }
            if win {
                v_win = true;
                break;
            }
        }

        // First diagonal wins
        let first = self.symbol_at(0, 0);
        let mut win = true;
        if let None = first {
            win = false;
        } else {
            for i in 0..3 {
                if self.symbol_at(i, i) != first {
                    win = false;
                    break
                }
            }
        }
        if win {
            d_win = true;
        }

        // Second diagonal wins
        let first = self.symbol_at(0, 2);
        let mut win = true;
        if let None = first {
            win = false;
        } else {
            for i in 0..3 {
                if self.symbol_at(i, 2 - i) != first {
                    win = false;
                    break
                }
            }
        }
        if win {
            d_win = true;
        }

        // Returns outcome relative to X player
        if h_win || v_win || d_win {
            Some(Outcome::Loss)
        } else if h_win || v_win || d_win {
            Some(Outcome::Win)
        } else if self.stack.len() == 9 {
            Some(Outcome::Tie)
        } else {
            None
        }
    }
}