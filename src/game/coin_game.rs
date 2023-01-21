// Max Fierro, maxfierro@berkeley.edu
// Friday January 20th, 2022

// A possible move.
pub enum Move {
    One,
    Two
}

// Represents the state of an ongoing game session.
#[derive(Clone, Copy)]
pub enum State {
    Win,
    Loss,
    Ongoing(i32)
}

// Represents an ongoing game session.
pub struct Session {
    state: State,
    turn: bool
}

impl Session {

    // Returns a new game. Turn should be true if you have the first turn.
    pub fn new(coins: i32) -> Self {
        Session {
            state: State::Ongoing(coins),
            turn: true
        }
    }

    // Returns the amount of coins left in the game.
    fn coins_left(&self) -> i32 {
        match self.state {
            State::Ongoing(num) => num,
            _ => 0
        }
    }

    // Returns true if it is your turn.
    fn my_turn(&self) -> bool {
        self.turn
    }

    // Indicates that the player's turn has ended.
    fn turn_over(&mut self) {
        self.turn = !self.turn
    }

    // Mutates the game's state according to the provided move. Returns
    // the resulting state.
    pub fn do_move(&mut self, mv: Move) -> State {
        let curr_coins = self.coins_left();
        match mv {
            Move::One => { 
                if curr_coins == 1 && self.my_turn() {
                    self.state = State::Win;
                } else if curr_coins == 1 {
                    self.state = State::Loss;
                } else {
                    self.state = State::Ongoing(curr_coins - 1);
                }
            },
            Move::Two => {
                if curr_coins == 2 && self.my_turn() {
                    self.state = State::Win;
                } else if curr_coins == 2 {
                    self.state = State::Loss;
                } else {
                    self.state = State::Ongoing(curr_coins - 2);
                }
            }
        }
        self.turn_over();
        self.state
    }

    // Returns an option of a vector containing possible moves derived from
    // the current state of the game session.
    pub fn possible_moves(&self) -> Option<Vec<Move>> {
        let coins_left = self.coins_left();
        if coins_left > 1 {
            Some(vec![Move::One, Move::Two])
        } else if coins_left == 1 {
            Some(vec![Move::One])
        } else {
            None
        }
    }
}